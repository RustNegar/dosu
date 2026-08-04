//! `dosu doctor` — a standalone environment diagnostic. Doesn't touch the
//! pty/Grid/render pipeline at all; just inspects config files and env
//! vars for the specific known-interaction issues this project has
//! actually hit in practice (documented in README.md's "تداخل‌های
//! شناخته‌شده" section), and prints copy-pasteable fixes.
//!
//! Deliberately conservative: never edits a file on its own. Always
//! prints the exact snippet and lets the person paste it in themselves,
//! since these are all things living in *their* dotfiles, not dosu's.

use std::path::{Path, PathBuf};

struct Check {
    name: &'static str,
    status: Status,
    detail: String,
}

enum Status {
    Ok,
    Warn,
    Info,
}

impl Status {
    fn symbol(&self) -> &'static str {
        match self {
            Status::Ok => "✅",
            Status::Warn => "⚠️ ",
            Status::Info => "ℹ️ ",
        }
    }
}

pub fn run() {
    println!("dosu doctor — بررسی محیط\n");

    let mut checks = Vec::new();
    checks.push(check_term_program());
    checks.push(check_kitty_force_ltr());
    checks.push(check_tmux_navigator());
    checks.push(check_fzf_widget_wrapper());
    checks.push(check_vi_mode());

    let mut warn_count = 0;
    for c in &checks {
        if matches!(c.status, Status::Warn) {
            warn_count += 1;
        }
        println!("{} {}", c.status.symbol(), c.name);
        if !c.detail.is_empty() {
            for line in c.detail.lines() {
                println!("   {line}");
            }
        }
        println!();
    }

    if warn_count == 0 {
        println!("همه‌چیز سالم به‌نظر می‌رسه.");
    } else {
        println!(
            "{warn_count} مورد نیاز به توجه داره — جزئیات بالا. dosu خودش هیچ فایلی رو تغییر نمی‌ده؛ خط‌های پیشنهادی رو خودت دستی اضافه کن."
        );
    }
}

fn home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

fn read_to_string(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

fn check_term_program() -> Check {
    let term_program = std::env::var("TERM_PROGRAM").unwrap_or_default();
    let known = ["ghostty", "kitty", "iTerm.app", "Apple_Terminal", "tmux", "WezTerm"];
    if term_program.is_empty() {
        Check {
            name: "ترمینال میزبان".into(),
            status: Status::Info,
            detail: "TERM_PROGRAM تشخیص داده نشد؛ نمی‌شه چک‌های مخصوصِ ترمینال رو انجام داد.".into(),
        }
    } else if known.iter().any(|k| term_program.contains(k)) {
        Check {
            name: format!("ترمینال میزبان: {term_program}").leak(),
            status: Status::Ok,
            detail: String::new(),
        }
    } else {
        Check {
            name: format!("ترمینال میزبان: {term_program} (ناشناخته)").leak(),
            status: Status::Info,
            detail: "این ترمینال قبلاً تست نشده؛ اگه چیزی عجیب دیدی، گزارش بده.".into(),
        }
    }
}

fn check_kitty_force_ltr() -> Check {
    let term_program = std::env::var("TERM_PROGRAM").unwrap_or_default();
    if !term_program.to_lowercase().contains("kitty") {
        return Check { name: "Kitty force_ltr".into(), status: Status::Ok, detail: String::new() };
    }
    let Some(home) = home() else {
        return Check {
            name: "Kitty force_ltr".into(),
            status: Status::Info,
            detail: "$HOME پیدا نشد، نمی‌شه kitty.conf رو چک کرد.".into(),
        };
    };
    let conf = home.join(".config/kitty/kitty.conf");
    let content = read_to_string(&conf).unwrap_or_default();
    let has_it = content
        .lines()
        .map(str::trim)
        .any(|l| l.starts_with("force_ltr") && l.split_whitespace().nth(1) == Some("yes"));
    if has_it {
        Check { name: "Kitty force_ltr".into(), status: Status::Ok, detail: String::new() }
    } else {
        Check {
            name: "Kitty force_ltr".into(),
            status: Status::Warn,
            detail: format!(
                "Kitty خودش یه heuristic bidi داره که با reorder دوسو تداخل می‌کنه.\nاین خط رو به {} اضافه کن:\n\n  force_ltr yes",
                conf.display()
            ),
        }
    }
}

fn check_tmux_navigator() -> Check {
    let Some(home) = home() else {
        return Check { name: "tmux navigator plugin".into(), status: Status::Ok, detail: String::new() };
    };
    let plugin_dir = home.join(".tmux/plugins/vim-tmux-navigator");
    if !plugin_dir.exists() {
        return Check { name: "tmux navigator plugin".into(), status: Status::Ok, detail: String::new() };
    }
    let conf = home.join(".tmux.conf");
    let content = read_to_string(&conf).unwrap_or_default();
    if content.contains("@navigator_active") {
        Check { name: "tmux navigator plugin".into(), status: Status::Ok, detail: String::new() }
    } else {
        Check {
            name: "tmux navigator plugin".into(),
            status: Status::Warn,
            detail: format!(
                "vim-tmux-navigator نصبه ولی @navigator_active توی {} پیدا نشد.\nCtrl+hjkl ممکنه داخل fzf/vim به‌درستی فوروارد نشه (ps -t <tty> در\nبعضی محیط‌ها شکننده‌ست: github.com/christoomey/vim-tmux-navigator/issues/417).\nراه‌حل کامل توی بخش «تداخل‌های شناخته‌شده» README.",
                conf.display()
            ),
        }
    }
}

fn check_fzf_widget_wrapper() -> Check {
    let Some(home) = home() else {
        return Check { name: "fzf widget wrapper".into(), status: Status::Ok, detail: String::new() };
    };
    if std::process::Command::new("which")
        .arg("fzf")
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false)
        == false
    {
        return Check { name: "fzf widget wrapper".into(), status: Status::Ok, detail: String::new() };
    }
    let zshrc = home.join(".zshrc");
    let content = read_to_string(&zshrc).unwrap_or_default();
    let has_tmux_navigator = home.join(".tmux/plugins/vim-tmux-navigator").exists();
    if !has_tmux_navigator {
        return Check { name: "fzf widget wrapper".into(), status: Status::Ok, detail: String::new() };
    }
    if content.contains("navigator_mark") {
        Check { name: "fzf widget wrapper".into(), status: Status::Ok, detail: String::new() }
    } else {
        Check {
            name: "fzf widget wrapper".into(),
            status: Status::Warn,
            detail: "fzf و vim-tmux-navigator هر دو هستن ولی wrapper مربوطه توی .zshrc\nپیدا نشد. جزئیات کامل توی بخش «تداخل‌های شناخته‌شده» README.".into(),
        }
    }
}

fn check_vi_mode() -> Check {
    let Some(home) = home() else {
        return Check { name: "zsh vi-mode".into(), status: Status::Ok, detail: String::new() };
    };
    let zshrc = home.join(".zshrc");
    let content = read_to_string(&zshrc).unwrap_or_default();
    let has_vi_mode = content
        .lines()
        .map(str::trim)
        .any(|l| l == "bindkey -v" || l.starts_with("bindkey -v "));
    if has_vi_mode {
        Check {
            name: "zsh vi-mode (bindkey -v)".into(),
            status: Status::Warn,
            detail: "پشتیبانی از vi-mode هنوز کامل نیست (احتمالاً به تغییر شکل\nکرسر/DECSCUSR مربوطه). اگه رفتار عجیبی دیدی، با DOSU_DEBUG_DIR\nیه لاگ بگیر و گزارش بده.".into(),
        }
    } else {
        Check { name: "zsh vi-mode".into(), status: Status::Ok, detail: String::new() }
    }
}

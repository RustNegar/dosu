//! Background, non-blocking self-update check (oh-my-zsh style).
//!
//! Design note on *when* the notice prints: `dosu` takes over the whole
//! real terminal (raw mode + `\x1b[2J\x1b[H`) almost immediately after
//! startup, then hands the screen to the child pty for the rest of the
//! session. A background task that finishes *after* that point has no
//! coherent place to print a notice -- it would land in the middle of
//! whatever the wrapped shell/program is drawing to the real terminal.
//! So:
//!
//!   - The "new version available" notice is only ever printed
//!     *synchronously*, from the *cached* last-known-version, before
//!     raw mode is enabled (`check_cached_and_notify`). No network call,
//!     effectively instant, so it never delays startup.
//!   - If the cache is missing or older than the configured interval, a
//!     fully-detached background task (`spawn_background_refresh`) hits
//!     the GitHub API (2s timeout) and refreshes the cache for the
//!     *next* invocation. It is never awaited, never blocks startup,
//!     and never touches stdout/stderr -- a slow/unreachable network
//!     silently does nothing.

use dosu_core::Config;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const RELEASES_API: &str = "https://api.github.com/repos/RustNegar/dosu/releases/latest";
const HTTP_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateCache {
    last_checked: u64,
    latest_known_version: String,
}

/// `DOSU_DISABLE_UPDATE_CHECK=1` (or any non-empty, non-"0" value)
/// disables the check entirely, regardless of `config.toml`. This is
/// checked separately from `Config` so it's a hard override for
/// restricted/offline/CI environments even if a config file says
/// otherwise.
fn disabled_by_env() -> bool {
    std::env::var("DOSU_DISABLE_UPDATE_CHECK")
        .map(|v| !v.is_empty() && v != "0")
        .unwrap_or(false)
}

fn cache_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("", "", "dosu")
        .map(|dirs| dirs.cache_dir().join("update_check.json"))
}

fn read_cache() -> Option<UpdateCache> {
    let path = cache_path()?;
    let s = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&s).ok()
}

fn write_cache(cache: &UpdateCache) {
    let Some(path) = cache_path() else { return };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(s) = serde_json::to_string(cache) {
        let _ = std::fs::write(path, s);
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Prints the "new version available" notice iff `latest` parses as
/// semver and is newer than the running binary's version. Never
/// errors -- an unparsable version string (e.g. a cache file from a
/// future format) just means no notice, not a crash.
fn maybe_notice(latest: &str) {
    let running = env!("CARGO_PKG_VERSION");
    let Ok(latest_v) = semver::Version::parse(latest.trim_start_matches('v')) else {
        return;
    };
    let Ok(running_v) = semver::Version::parse(running) else {
        return;
    };
    if latest_v > running_v {
        eprintln!(
            "⚠ A new version of dosu is available: {latest_v} (you have {running_v}). Run `dosu update` or see https://github.com/RustNegar/dosu/releases"
        );
    }
}

/// Synchronous, network-free: reads the cache (if any) and prints the
/// notice if it says a newer version is available. Call this *before*
/// raw mode / the screen clear, at the very top of `main()`.
pub fn check_cached_and_notify(config: &Config) {
    if disabled_by_env() || !config.update_check_enabled {
        return;
    }
    if let Some(cache) = read_cache() {
        maybe_notice(&cache.latest_known_version);
    }
}

/// Fire-and-forget: if the cache is missing or older than
/// `config.update_check_interval_days`, spawns a background tokio task
/// that hits the GitHub Releases API and refreshes the cache for the
/// *next* run. Must be called from within a running Tokio runtime
/// (i.e. inside `async fn main`); never awaited by the caller.
pub fn spawn_background_refresh(config: &Config) {
    if disabled_by_env() || !config.update_check_enabled {
        return;
    }

    let interval_secs = config
        .update_check_interval_days
        .saturating_mul(24 * 60 * 60);
    let stale = match read_cache() {
        Some(cache) => now_unix().saturating_sub(cache.last_checked) >= interval_secs,
        None => true,
    };
    if !stale {
        return;
    }

    tokio::spawn(async move {
        // The actual HTTP call is blocking (ureq); run it on a
        // blocking-pool thread so it never occupies an async worker,
        // and so a hung DNS lookup etc. can't stall other tasks.
        let result = tokio::task::spawn_blocking(fetch_latest_tag).await;
        let Ok(Ok(tag)) = result else {
            return; // network error, timeout, or task panic: stay silent
        };
        write_cache(&UpdateCache {
            last_checked: now_unix(),
            latest_known_version: tag.trim_start_matches('v').to_string(),
        });
    });
}

fn fetch_latest_tag() -> anyhow::Result<String> {
    let agent = ureq::AgentBuilder::new().timeout(HTTP_TIMEOUT).build();
    let body: serde_json::Value = agent
        .get(RELEASES_API)
        .set("User-Agent", "dosu-update-check")
        .call()?
        .into_json()?;
    body.get("tag_name")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| anyhow::anyhow!("no tag_name field in GitHub releases response"))
}

/// `dosu update`: dosu never replaces its own binary in place -- that's
/// a lot of risk (partial writes, permissions, running-binary-on-disk
/// weirdness) for little benefit over just telling the user the one
/// command to run for however they installed it.
pub fn run_update_command() {
    let via_homebrew = detect_homebrew_install();

    println!("dosu doesn't self-update automatically -- run one of these:\n");
    if via_homebrew {
        println!("  Homebrew (detected): brew upgrade dosu");
        println!(
            "  Install script:      curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/install.sh | sh"
        );
    } else {
        println!(
            "  Install script (detected): curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/install.sh | sh"
        );
        println!("  Homebrew (macOS):           brew upgrade dosu");
    }
    println!("\nRelease notes: https://github.com/RustNegar/dosu/releases");
}

/// Best-effort detection of a Homebrew install: checks whether the
/// running binary's own path sits under a Homebrew prefix (`Cellar` or
/// `homebrew`), which is how `brew install`-managed binaries are laid
/// out on both Apple Silicon (`/opt/homebrew`) and Intel
/// (`/usr/local`) Macs, plus Linuxbrew. Not authoritative -- if it's
/// unclear, `run_update_command` prints both options anyway.
fn detect_homebrew_install() -> bool {
    std::env::current_exe()
        .ok()
        .map(|p| p.to_string_lossy().to_lowercase())
        .map(|p| p.contains("cellar") || p.contains("homebrew") || p.contains("linuxbrew"))
        .unwrap_or(false)
}

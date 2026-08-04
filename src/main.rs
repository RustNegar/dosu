//! Dosu CLI entry point.
//!
//! Ties the core pieces together:
//!   real tty (raw mode) <-> PtySession <-> vte::Parser -> Grid
//!                                                            |
//!                                                      bidi::reorder_grid
//!                                                            |
//!                                                       Renderer -> real tty

mod doctor;

use anyhow::Result;
use clap::Parser as ClapParser;
use crossterm::terminal;
use dosu_core::altscreen::{AltScreenScanner, Segment};
use dosu_core::bidi::{reorder_grid, NoopShaper};
use dosu_core::grid::{feed, Grid};
use dosu_core::{Config, PtySession, PtyWriterHandle, Renderer};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};

/// Debug tap, enabled by setting `DOSU_DEBUG_DIR=/some/dir`. Writes three
/// separate append-only logs so the three legs of the pipeline can be
/// told apart (a `script -q ... dosu` capture only ever sees
/// `dosu_to_terminal.log`, i.e. what Dosu decided to draw):
///   - `child_to_dosu.log`     raw bytes from the child's pty, before the
///                             alt-screen scanner or vte parser touch them.
///   - `dosu_to_child.log`     bytes Dosu writes back to the child
///                             (currently just terminal-query replies).
///   - `dosu_to_terminal.log`  bytes Dosu writes to the real terminal
///                             (render diffs + raw alt-screen passthrough).
/// Each write is preceded by a `=== <label> len=N ===` marker line.
#[derive(Clone)]
struct DebugTap {
    child_to_dosu: Option<Arc<Mutex<File>>>,
    dosu_to_child: Option<Arc<Mutex<File>>>,
    dosu_to_terminal: Option<Arc<Mutex<File>>>,
}

impl DebugTap {
    fn from_env() -> Self {
        let dir = match std::env::var("DOSU_DEBUG_DIR") {
            Ok(d) => d,
            Err(_) => {
                return DebugTap {
                    child_to_dosu: None,
                    dosu_to_child: None,
                    dosu_to_terminal: None,
                }
            }
        };
        let _ = std::fs::create_dir_all(&dir);
        let open = |name: &str| -> Option<Arc<Mutex<File>>> {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(format!("{dir}/{name}"))
                .ok()
                .map(|f| Arc::new(Mutex::new(f)))
        };
        DebugTap {
            child_to_dosu: open("child_to_dosu.log"),
            dosu_to_child: open("dosu_to_child.log"),
            dosu_to_terminal: open("dosu_to_terminal.log"),
        }
    }

    fn log(file: &Option<Arc<Mutex<File>>>, label: &str, bytes: &[u8]) {
        if let Some(f) = file {
            if let Ok(mut f) = f.lock() {
                let _ = write!(f, "\n=== {label} len={} ===\n", bytes.len());
                let _ = f.write_all(bytes);
                let _ = f.flush();
            }
        }
    }
}

#[derive(ClapParser, Debug)]
#[command(name = "dosu", about = "A modern bidirectional (Persian/Arabic) terminal wrapper")]
struct Args {
    /// Command to run instead of the default shell.
    command: Option<String>,
}

/// Wraps a writer (locked stdout) and mirrors every write into the
/// `dosu_to_terminal` debug log, when configured -- lets both the normal
/// render path and the raw alt-screen passthrough share one complete log.
struct TeeOut<'a> {
    inner: io::StdoutLock<'a>,
    tap: Option<Arc<Mutex<File>>>,
}

impl<'a> Write for TeeOut<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        DebugTap::log(&self.tap, "dosu_to_terminal", &buf[..n]);
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

fn term_size() -> (u16, u16) {
    match terminal::size() {
        Ok((c, r)) if c > 0 && r > 0 => (c, r),
        _ => (80, 24),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::args().nth(1).as_deref() == Some("doctor") {
        doctor::run();
        return Ok(());
    }

    let args = Args::parse();
    let config = Config::load();
    let filter = tracing_subscriber::EnvFilter::try_new(&config.log_level)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // CLI arg wins; config.toml's `shell` is the fallback before $SHELL
    // (which PtySession::spawn falls back to on its own for `None`).
    let command = args.command.or_else(|| config.shell.clone());

    // Guard against nested re-entry (see zsh integration).
    if std::env::var("DOSU").is_ok() {
        eprintln!("dosu: already running inside dosu, skipping.");
        let shell = command.unwrap_or_else(|| std::env::var("SHELL").unwrap_or("/bin/zsh".into()));
        return Err(anyhow::anyhow!("refusing nested launch; run `{shell}` directly instead"));
    }

    let (cols, term_rows) = term_size();

    terminal::enable_raw_mode()?;
    let _raw_guard = scopeguard(|| {
        let _ = terminal::disable_raw_mode();
    });

    // dosu owns the whole real terminal from row 1: no DSR cursor
    // query, no row_offset tracking of pre-existing content above us --
    // just clear and start at top-left. Trades preserving prior screen
    // content for removing the "real scroll vs. row_offset bookkeeping"
    // bug class.
    let stdout_handle = io::stdout();
    {
        let mut out = stdout_handle.lock();
        out.write_all(b"\x1b[2J\x1b[H")?;
        out.flush()?;
    }
    drop(stdout_handle);
    let row_offset = 0usize;
    let rows = term_rows;

    let pty = Arc::new(PtySession::spawn(command.as_deref(), cols, rows)?);
    let mut pty_reader = pty.try_clone_reader()?;
    // The reader task writes terminal-query replies (DSR, etc.) straight
    // back to the child's stdin, never to our own stdout -- grab a
    // shared handle now since `pty` moves into the stdin task below.
    let pty_writer_for_reader: PtyWriterHandle = pty.writer_handle();
    let debug_tap = DebugTap::from_env();
    if debug_tap.child_to_dosu.is_some() {
        eprintln!("dosu: DOSU_DEBUG_DIR set, logging raw pty traffic to that directory.");
    }
    let debug_tap_for_reader = debug_tap.clone();

    let grid = Arc::new(Mutex::new(Grid::new(cols as usize, rows as usize)));
    let renderer = Arc::new(Mutex::new(Renderer::new()));
    renderer.lock().unwrap().set_row_offset(row_offset, term_rows as usize);
    let mut parser = vte::Parser::new();

    // Task: pty -> parse -> bidi -> render.
    let grid_for_reader = grid.clone();
    let renderer_for_reader = renderer.clone();
    let reader_task = tokio::task::spawn_blocking(move || -> Result<()> {
        let mut buf = [0u8; 8192];
        let stdout = io::stdout();
        let mut scanner = AltScreenScanner::new();
        let pty_writer = pty_writer_for_reader;
        let debug_tap = debug_tap_for_reader;
        loop {
            let n = match pty_reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            DebugTap::log(&debug_tap.child_to_dosu, "child_to_dosu", &buf[..n]);

            for segment in scanner.scan(&buf[..n]) {
                match segment {
                    Segment::Grid(bytes) => {
                        let mut g = grid_for_reader.lock().unwrap();
                        let responses = feed(&mut parser, &mut g, &bytes);
                        if !responses.is_empty() {
                            // A terminal-query reply (currently just DSR
                            // cursor-position) goes to the child, never
                            // to our own stdout.
                            DebugTap::log(&debug_tap.dosu_to_child, "dosu_to_child", &responses);
                            let mut w = pty_writer.lock().unwrap();
                            w.write_all(&responses)?;
                            w.flush()?;
                        }
                        let scroll_lines = g.take_scroll_lines();
                        let visual = reorder_grid(&g, &NoopShaper);
                        let mut out = TeeOut { inner: stdout.lock(), tap: debug_tap.dosu_to_terminal.clone() };
                        renderer_for_reader
                            .lock()
                            .unwrap()
                            .render(&visual, &mut out, scroll_lines)?;
                    }
                    Segment::Raw(bytes) => {
                        // Alt-screen content (neovim, tmux, less, ...):
                        // the child drives the real terminal directly.
                        // Don't touch Grid while this is happening.
                        let mut out = TeeOut { inner: stdout.lock(), tap: debug_tap.dosu_to_terminal.clone() };
                        out.write_all(&bytes)?;
                        out.flush()?;
                        if !scanner.in_alt_screen() {
                            // Just left the alt screen. The real terminal
                            // restored its own saved primary buffer,
                            // which should match our untouched Grid --
                            // invalidate anyway so any drift is corrected
                            // next frame instead of compounding.
                            renderer_for_reader.lock().unwrap().invalidate();
                        }
                    }
                }
            }
        }
        Ok(())
    });

    // Task: real terminal resize (SIGWINCH) -> resize both the child pty
    // and our Grid, then force a full repaint. Without this, a window/
    // tmux-pane resize leaves Dosu computing positions against stale
    // dimensions for the rest of the session.
    let pty_for_resize = pty.clone();
    let grid_for_resize = grid.clone();
    let renderer_for_resize = renderer.clone();
    let resize_task = tokio::task::spawn(async move {
        let mut sig = match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::window_change()) {
            Ok(s) => s,
            Err(_) => return, // resize notifications unavailable; degrade silently
        };
        loop {
            if sig.recv().await.is_none() {
                break;
            }
            let (cols, term_rows) = term_size();
            let mut g = grid_for_resize.lock().unwrap();
            // `g.rows`/`g.cols` are dosu's grid dimensions, deliberately
            // smaller than the real terminal by `row_offset` (the header
            // space above dosu's drawable area -- see startup sizing in
            // `run()`). Comparing against the raw terminal size instead
            // made every SIGWINCH look like a real resize and handed the
            // grid `row_offset` extra rows -- rows past the real
            // terminal's bottom edge once `row + 1 + row_offset`
            // addressing applies, silently clamped by the real terminal.
            // That caused dosu to appear to run out of screen and start
            // scrolling early.
            let row_offset = renderer_for_resize.lock().unwrap().row_offset();
            let rows = term_rows.saturating_sub(row_offset as u16).max(1);
            // SIGWINCH can fire without dimensions actually changing
            // (common here given how often tmux status-bar plugins
            // redraw). `Grid::resize` unconditionally resets DECSTBM
            // margins/wrapped_rows/structured_rows even for a same-size
            // call, so treating every SIGWINCH as real would wipe an
            // in-progress restricted scroll region (e.g. fzf's preview
            // pane) on a spurious signal, misreading later scrolls in
            // that region as full-page scrolls pushed into scrollback.
            if g.cols == cols as usize && g.rows == rows as usize {
                continue;
            }
            let _ = pty_for_resize.resize(cols, rows);
            let stdout = io::stdout();
            let mut out = stdout.lock();
            {
                g.resize(cols as usize, rows as usize);
                let visual = reorder_grid(&g, &NoopShaper);
                let mut r = renderer_for_resize.lock().unwrap();
                r.set_row_offset(row_offset, term_rows as usize);
                r.invalidate();
                let _ = r.render(&visual, &mut out, 0);
            }
        }
    });

    // Task: stdin -> pty (raw passthrough; macOS's own Persian/Arabic
    // keyboard layouts already produce correct UTF-8, no keymap
    // reimplementation needed).
    let stdin_task = tokio::task::spawn_blocking(move || -> Result<()> {
        let mut stdin = io::stdin();
        let mut buf = [0u8; 4096];
        loop {
            let n = match stdin.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            pty.write_input(&buf[..n])?;
        }
        Ok(())
    });

    let result: Result<()> = tokio::select! {
        r = reader_task => r.map_err(anyhow::Error::from).and_then(|x| x),
        r = stdin_task => r.map_err(anyhow::Error::from).and_then(|x| x),
        _ = resize_task => Ok(()),
    };

    // Restore the terminal explicitly and hard-exit here rather than
    // letting `main()` return normally: `stdin_task` is a spawn_blocking
    // OS thread stuck in a blocking `stdin.read()`, which nothing but
    // more keyboard input can unblock. Tokio's default shutdown waits
    // for every spawned blocking task to finish, so once the shell
    // exits the process would hang until that stray read() unblocks --
    // this was the "need to press Enter twice" bug (the second Enter
    // just frees the stuck thread). `std::process::exit` skips
    // destructors, so raw mode is disabled by hand first.
    let _ = terminal::disable_raw_mode();
    match result {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("dosu: {e}");
            std::process::exit(1);
        }
    }
}

/// Tiny inline scopeguard so we don't pull in the `scopeguard` crate for
/// one use site.
fn scopeguard<F: FnOnce()>(f: F) -> impl Drop {
    struct Guard<F: FnOnce()>(Option<F>);
    impl<F: FnOnce()> Drop for Guard<F> {
        fn drop(&mut self) {
            if let Some(f) = self.0.take() {
                f();
            }
        }
    }
    Guard(Some(f))
}

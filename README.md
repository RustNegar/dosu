<div align="center">
  <img src="assets/logo.png" alt="Dosu Logo" width="150" height="150">
  <h1>Dosu</h1>
  <p>A modern, cross-platform bidirectional terminal wrapper — the CLI of the RustNegar project</p>
  <br>
  <p>
    <img src="https://img.shields.io/badge/rust-+v1.7-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
    <img src="https://img.shields.io/badge/version-0.1.0-22C8E6?style=for-the-badge" alt="Version">
    <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" alt="License">
    <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-lightgrey?style=for-the-badge" alt="Platform">
  </p>
  <br>
  <p>
    <b>English</b> •
    <a href="README_FA.md" dir="rtl">فارسی</a> •
    <a href="README_AR.md" dir="rtl">العربية</a> •
    <a href="README_HE.md" dir="rtl">עברית</a>
  </p>
  <br>
  <table>
    <tr>
      <td align="center" width="110"><a href="#about"><img src="assets/icons/about.svg" width="44" alt="About"/><br/><sub><b>About</b></sub></a></td>
      <td align="center" width="110"><a href="#installation"><img src="assets/icons/install.svg" width="44" alt="Install"/><br/><sub><b>Install</b></sub></a></td>
      <td align="center" width="110"><a href="#support-the-project---donate"><img src="assets/icons/donate.svg" width="44" alt="Donate"/><br/><sub><b>Donate</b></sub></a></td>
      <td align="center" width="110"><a href="#usage"><img src="assets/icons/usage.svg" width="44" alt="Usage"/><br/><sub><b>Usage</b></sub></a></td>
      <td align="center" width="110"><a href="#configuration"><img src="assets/icons/config.svg" width="44" alt="Config"/><br/><sub><b>Config</b></sub></a></td>
      <td align="center" width="110"><a href="#known-issues"><img src="assets/icons/issues.svg" width="44" alt="Issues"/><br/><sub><b>Issues</b></sub></a></td>
      <td align="center" width="110"><a href="#contact"><img src="assets/icons/contact.svg" width="44" alt="Contact"/><br/><sub><b>Contact</b></sub></a></td>
    </tr>
  </table>
</div>

<br>

## About

**Dosu** (دوسو) is a sophisticated terminal wrapper designed to handle bidirectional text rendering seamlessly. Built with Rust on top of [`dosu-core`](https://github.com/RustNegar/dosu-core), it addresses the complex challenges of displaying right-to-left (RTL) languages like Persian and Arabic in terminal environments — without breaking the tools you already use.

<br>

## Installation

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/install.sh | sh
```

### Homebrew (macOS)

```bash
brew install rustnegar/dosu/dosu
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/RustNegar/dosu.git
cd dosu

# Build in release mode
cargo build --release

# Install binary
cargo install --path .
```

### Requirements

- Rust 1.70 or higher (only needed to build from source)
- A compatible terminal emulator (Kitty, iTerm2, Ghostty, WezTerm, etc.)
- A Unix-like operating system (Linux or macOS)

<br>

## Usage

### Basic Usage

Simply run `dosu` to start a new bidirectional terminal session:

```bash
dosu
```

### Diagnostic Command

Run the built-in diagnostic tool to check your environment:

```bash
dosu doctor
```

This checks for:

- Terminal compatibility
- Known configuration conflicts (Kitty, tmux, vi-mode)
- Environment variable setup
- Recommended fixes

### Debug Mode

Enable detailed logging for development:

```bash
export DOSU_DEBUG_DIR=/tmp/dosu-debug
dosu
```

This creates three log files:

| File                   | Contents                         |
| ---------------------- | -------------------------------- |
| `child_to_dosu.log`    | Raw bytes from the child process |
| `dosu_to_child.log`    | Bytes written back to the child  |
| `dosu_to_terminal.log` | Render output to the terminal    |

### Update Check

`dosu` periodically checks GitHub Releases for a newer version (every 7 days by default), similar to oh-my-zsh. The check never blocks startup: it reads a small cache file, prints a short notice if you're behind, and refreshes that cache in the background for next time. If it finds a newer version:

```
⚠ A new version of dosu is available: 0.2.0 (you have 0.1.0). Run `dosu update` or see https://github.com/RustNegar/dosu/releases
```

Run `dosu update` to see the right upgrade command for how you installed it:

```bash
dosu update
```

To disable the check entirely (useful in restricted/offline/CI environments):

```bash
export DOSU_DISABLE_UPDATE_CHECK=1
```

It can also be disabled, or the interval changed, via `config.toml` (see [Configuration](#configuration)).

### Command Line Options

```bash
dosu --help
```

<br>

## Configuration

Dosu works out of the box with sensible defaults. However, you may need to adjust your terminal or shell configuration for the best experience.

Settings live in `~/.config/dosu/config.toml` (created only if you add it yourself). Relevant to the update check:

```toml
update_check_enabled = true       # set false to disable (or use DOSU_DISABLE_UPDATE_CHECK=1)
update_check_interval_days = 7    # how often to check GitHub Releases
```

### Recommended Terminal Settings

- **Font** — use a font with good RTL support (e.g. Vazirmatn, Fira Code with Nerd Font)
- **Locale** — make sure your locale supports UTF-8 (`LANG=en_US.UTF-8`)
- **Direction** — some terminals may require explicit RTL configuration

### Shell Integration

Add the RustNegar zsh helper to your shell config:

```bash
curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/zsh/rustnegar.zsh >> ~/.zshrc
```

Or add a quick alias to `~/.bashrc` / `~/.zshrc`:

```bash
alias d='dosu'
```

<br>

## Known Issues

Dosu has been tested extensively, but certain terminal/shell combinations may need manual configuration:

<table>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="Issue"/></td>
    <td><strong>Kitty Terminal</strong><br/>May require a <code>force_ltr</code> adjustment in <code>kitty.conf</code>.</td>
  </tr>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="Issue"/></td>
    <td><strong>tmux</strong><br/>Plugin conflicts are possible — check your <code>tmux-navigator</code> settings.</td>
  </tr>
  <tr>
    <td width="65" align="center"><img src="assets/icons/issues.svg" width="28" alt="Issue"/></td>
    <td><strong>FZF Widget</strong><br/>Custom wrappers may need adjustment for RTL.</td>
  </tr>
</table>

Run `dosu doctor` for automatic detection and fix recommendations.

For detailed troubleshooting, see the [Wiki](https://github.com/RustNegar/dosu/wiki/Troubleshooting).

<br>

## Contact

<table>
  <tr>
    <td align="center" width="65"><img src="assets/icons/contact.svg" width="36" alt="Contact"/></td>
    <td>
      <strong>Repository</strong>: <a href="https://github.com/RustNegar/dosu">github.com/RustNegar/dosu</a><br/>
      <strong>Core Engine</strong>: <a href="https://github.com/RustNegar/dosu-core">github.com/RustNegar/dosu-core</a><br/>
      <strong>Author</strong>: Kurosh Mirhajian<br/>
      <strong>License</strong>: MIT
    </td>
  </tr>
</table>

<br>

## Support the Project - Donate

If Dosu has been useful to you, consider supporting its continued development. Every contribution, big or small, is deeply appreciated.

<div align="center">

| Network                                                                                                          | Address                                            |
| :--------------------------------------------------------------------------------------------------------------- | :------------------------------------------------- |
| ![TON](https://img.shields.io/badge/TON-0088CC?style=flat-square&logo=ton&logoColor=white)                       | `UQDPxrimgBU6Mil0dhDn0Fc303RLRXKr9hGGDu7bTEBdGGqs` |
| ![TRC20](<https://img.shields.io/badge/TRC20%20(Tron)-FF060A?style=flat-square&logo=tron&logoColor=white>)       | `TXix7uf6JPUKvWeUbA4A7wmQLVKnDbLRQU`               |
| ![ETH](<https://img.shields.io/badge/ERC20%20(Ethereum)-3C3C3D?style=flat-square&logo=ethereum&logoColor=white>) | `0x1FC907d3396460f1Cd94E3BC48564b1b46b70026`       |

</div>

<br>

<div align="center">
  <p>Built with ❤️ using Rust</p>
  <p>
    <img src="https://img.shields.io/badge/Made%20with-Rust-orange?style=flat-square&logo=rust&logoColor=white" alt="Made with Rust">
  </p>
</div>

#!/bin/sh
# dosu installer.
#
#   curl -fsSL https://raw.githubusercontent.com/RustNegar/dosu/main/install.sh | sh
#
# Detects OS/arch, downloads the matching release tarball, verifies its
# checksum, and installs the `dosu` binary. POSIX sh (not bash) so it
# also runs correctly under `sh` and dash, not just bash.
set -eu

REPO="RustNegar/dosu"
INSTALL_DIR="${DOSU_INSTALL_DIR:-$HOME/.local/bin}"

info()  { printf '\033[1;34m==>\033[0m %s\n' "$1"; }
warn()  { printf '\033[1;33mwarning:\033[0m %s\n' "$1" >&2; }
die()   { printf '\033[1;31merror:\033[0m %s\n' "$1" >&2; exit 1; }

need_cmd() {
    command -v "$1" >/dev/null 2>&1 || die "'$1' is required but not installed."
}

need_cmd curl
need_cmd tar

# --- detect platform ---------------------------------------------------
os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
    Darwin) os_target="apple-darwin" ;;
    Linux)  os_target="unknown-linux-gnu" ;;
    *) die "unsupported OS: $os (dosu currently ships macOS and Linux builds)" ;;
esac

case "$arch" in
    x86_64|amd64)   arch_target="x86_64" ;;
    arm64|aarch64)  arch_target="aarch64" ;;
    *) die "unsupported architecture: $arch" ;;
esac

target="${arch_target}-${os_target}"
asset="dosu-${target}.tar.gz"

# --- resolve version -----------------------------------------------------
if [ -n "${DOSU_VERSION:-}" ]; then
    version="$DOSU_VERSION"
else
    info "Looking up the latest release..."
    version="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name":' | head -n1 | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')"
    [ -n "$version" ] || die "could not determine the latest release version"
fi

url="https://github.com/${REPO}/releases/download/${version}/${asset}"
checksum_url="${url}.sha256"

info "Installing dosu ${version} for ${target}..."

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT INT TERM

curl -fsSL -o "$tmpdir/$asset" "$url" \
    || die "download failed: $url (does this release ship a build for $target?)"
curl -fsSL -o "$tmpdir/$asset.sha256" "$checksum_url" \
    || die "download failed: $checksum_url"

# --- verify checksum -------------------------------------------------
(
    cd "$tmpdir"
    if command -v shasum >/dev/null 2>&1; then
        shasum -a 256 -c "$asset.sha256"
    elif command -v sha256sum >/dev/null 2>&1; then
        sha256sum -c "$asset.sha256"
    else
        warn "no sha256sum/shasum found; skipping checksum verification"
    fi
) || die "checksum verification failed -- the downloaded file may be corrupt"

# --- install -----------------------------------------------------------
tar -xzf "$tmpdir/$asset" -C "$tmpdir"
extracted_dir="$tmpdir/dosu-${target}"

mkdir -p "$INSTALL_DIR"
cp "$extracted_dir/dosu" "$INSTALL_DIR/dosu"
chmod +x "$INSTALL_DIR/dosu"

# curl (unlike a browser) doesn't set com.apple.quarantine, but clear it
# defensively in case it was set some other way (e.g. AirDrop, Mail).
if [ "$os" = "Darwin" ] && command -v xattr >/dev/null 2>&1; then
    xattr -d com.apple.quarantine "$INSTALL_DIR/dosu" 2>/dev/null || true
fi

info "Installed to $INSTALL_DIR/dosu"

case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        warn "$INSTALL_DIR is not on your PATH."
        echo "  Add this to your shell config:"
        echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
        ;;
esac

echo
info "To launch dosu automatically in new zsh sessions, add this to ~/.zshrc:"
echo "    curl -fsSL https://raw.githubusercontent.com/${REPO}/main/zsh/rustnegar.zsh >> ~/.zshrc"
echo
info "Done. Run 'dosu' to start."

#!/bin/sh
# Installs easy-peasy-mcp on Linux and macOS, see INSTALL.md.
#
# Builds the binary from the source checkout this script lives in, or downloads a release binary from GitHub when
# there is no checkout or `--download` is given, and copies it into the install directory.
set -eu

REPO="kefirfromperm/easy-peasy-mcp"
NAME="easy-peasy-mcp"

usage() {
    cat <<EOF
Usage: install.sh [--dir <dir>] [--build | --download] [--version <tag>] [--help]

  --dir <dir>       install directory (default: \$EASY_PEASY_INSTALL_DIR or ~/.local/bin)
  --build           build from the source checkout with cargo
  --download        download a release binary from GitHub
  --version <tag>   release to download, e.g. v0.121.0 (default: the latest release)

Without --build or --download the script builds when it runs from a source checkout and cargo is installed,
and downloads otherwise.
EOF
}

die() {
    echo "install.sh: $*" >&2
    exit 1
}

dir="${EASY_PEASY_INSTALL_DIR:-$HOME/.local/bin}"
method=""
version=""

while [ $# -gt 0 ]; do
    case "$1" in
        --dir) [ $# -ge 2 ] || die "--dir needs a value"; dir="$2"; shift 2 ;;
        --build) method="build"; shift ;;
        --download) method="download"; shift ;;
        --version) [ $# -ge 2 ] || die "--version needs a value"; version="$2"; shift 2 ;;
        -h|--help) usage; exit 0 ;;
        *) usage >&2; exit 2 ;;
    esac
done

src="$(cd "$(dirname "$0")" && pwd)"

# a source checkout of this project, not any Rust project the script happens to run in (e.g. piped to sh)
checkout() {
    grep -q "^name = \"$NAME\"" "$src/Cargo.toml" 2>/dev/null
}

if [ -z "$method" ]; then
    if checkout && command -v cargo >/dev/null 2>&1; then
        method="build"
    else
        method="download"
    fi
fi

# the release target of this machine, as named by .github/workflows/release.yml
target() {
    os="$(uname -s)"
    arch="$(uname -m)"
    case "$os/$arch" in
        Linux/x86_64|Linux/amd64) echo "x86_64-unknown-linux-gnu" ;;
        Darwin/arm64|Darwin/aarch64) echo "aarch64-apple-darwin" ;;
        Darwin/x86_64) echo "x86_64-apple-darwin" ;;
        *) die "no release binary for $os $arch; build from source: install.sh --build" ;;
    esac
}

fetch() {
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$1"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$1"
    else
        die "curl or wget is needed to download"
    fi
}

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
trap 'exit 130' INT TERM

case "$method" in
    build)
        checkout || die "no source checkout next to the script; use --download"
        command -v cargo >/dev/null 2>&1 || die "cargo is not installed, see https://rustup.rs"
        echo "Building $NAME in $src ..."
        (cd "$src" && cargo build --release --locked)
        bin="$src/target/release/$NAME"
        ;;
    download)
        t="$(target)"
        if [ -z "$version" ]; then
            version="$(fetch "https://api.github.com/repos/$REPO/releases/latest" \
                | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -n 1)"
            [ -n "$version" ] || die "can't find the latest release of $REPO"
        fi
        archive="$NAME-$version-$t"
        echo "Downloading $archive.tar.gz ..."
        fetch "https://github.com/$REPO/releases/download/$version/$archive.tar.gz" > "$tmp/$archive.tar.gz" \
            || die "can't download $archive.tar.gz"
        tar xzf "$tmp/$archive.tar.gz" -C "$tmp"
        bin="$tmp/$archive/$NAME"
        ;;
esac

[ -f "$bin" ] || die "the binary $bin wasn't produced"

mkdir -p "$dir"
# install to a temporary name and rename, so a running server keeps its old binary
cp "$bin" "$dir/.$NAME.new"
chmod 755 "$dir/.$NAME.new"
mv -f "$dir/.$NAME.new" "$dir/$NAME"

echo "Installed: $("$dir/$NAME" --version) -> $dir/$NAME"

case ":$PATH:" in
    *":$dir:"*) ;;
    *) echo "Note: $dir is not in PATH; add it, or register the server with the full path $dir/$NAME" ;;
esac

cat <<EOF

Register it in an MCP client, e.g. Claude Code:

  claude mcp add easy-peasy -- $dir/$NAME --root ~/projects

See INSTALL.md for other clients and the config file.
EOF

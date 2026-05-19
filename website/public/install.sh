#!/usr/bin/env bash
# mimo-tui · one-liner installer
#
#   curl -fsSL https://mimo-tui.pages.dev/install.sh | sh
#
# Env vars:
#   MIMO_VERSION=v0.2.0-alpha.2   (override version)
#   MIMO_INSTALL_DIR=/usr/local/bin (override install dir)

set -euo pipefail

REPO="duolaAmengweb3/mimo-tui"
VERSION="${MIMO_VERSION:-v0.2.0-alpha.2}"
INSTALL_DIR="${MIMO_INSTALL_DIR:-}"

# ---------------------------------------------------------- platform detection

uname_os="$(uname -s | tr '[:upper:]' '[:lower:]')"
uname_arch="$(uname -m)"

case "$uname_os" in
  darwin)
    os=apple-darwin
    archive_ext=tar.gz
    ;;
  linux)
    if ldd /bin/sh 2>/dev/null | grep -q musl; then
      os=unknown-linux-musl
    else
      os=unknown-linux-gnu
    fi
    archive_ext=tar.gz
    ;;
  msys*|mingw*|cygwin*)
    os=pc-windows-msvc
    archive_ext=zip
    ;;
  *)
    echo "[mimo-tui] unsupported OS: $uname_os" >&2
    exit 1
    ;;
esac

case "$uname_arch" in
  x86_64|amd64) arch=x86_64 ;;
  arm64|aarch64) arch=aarch64 ;;
  *)
    echo "[mimo-tui] unsupported arch: $uname_arch" >&2
    exit 1
    ;;
esac

target="${arch}-${os}"
asset="mimo-${target}.${archive_ext}"
url="https://github.com/${REPO}/releases/download/${VERSION}/${asset}"
sha_url="${url}.sha256"

echo "[mimo-tui] installing ${VERSION} for ${target}"
echo "[mimo-tui] url: ${url}"

# ---------------------------------------------------------- install dir
if [ -z "${INSTALL_DIR}" ]; then
  if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR=/usr/local/bin
  else
    INSTALL_DIR="${HOME}/.local/bin"
    mkdir -p "${INSTALL_DIR}"
  fi
fi
echo "[mimo-tui] install dir: ${INSTALL_DIR}"

# ---------------------------------------------------------- download
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

curl -fsSL "$url" -o "${tmp}/${asset}"

# ---------------------------------------------------------- verify (best-effort)
if curl -fsSL "$sha_url" -o "${tmp}/${asset}.sha256" 2>/dev/null; then
  expected=$(awk '{print $1}' < "${tmp}/${asset}.sha256")
  if command -v shasum >/dev/null 2>&1; then
    actual=$(shasum -a 256 "${tmp}/${asset}" | awk '{print $1}')
  else
    actual=$(sha256sum "${tmp}/${asset}" | awk '{print $1}')
  fi
  if [ "$expected" != "$actual" ]; then
    echo "[mimo-tui] sha256 mismatch · expected $expected · got $actual" >&2
    exit 1
  fi
  echo "[mimo-tui] sha256 ok"
fi

# ---------------------------------------------------------- extract + install
case "$archive_ext" in
  tar.gz) tar xzf "${tmp}/${asset}" -C "$tmp" ;;
  zip)    unzip -q "${tmp}/${asset}" -d "$tmp" ;;
esac

binname=mimo
[ "$os" = "pc-windows-msvc" ] && binname=mimo.exe
src="${tmp}/mimo-${target}/${binname}"

if [ ! -f "$src" ]; then
  echo "[mimo-tui] could not find ${binname} in extracted archive" >&2
  exit 1
fi

install -m 0755 "$src" "${INSTALL_DIR}/${binname}"

# ---------------------------------------------------------- success
echo ""
echo "[mimo-tui] installed → ${INSTALL_DIR}/${binname}"

case ":$PATH:" in
  *":${INSTALL_DIR}:"*) ;;
  *)
    echo ""
    echo "[mimo-tui] ${INSTALL_DIR} is not in your PATH."
    echo "[mimo-tui] add this to your shell rc:"
    echo "             export PATH=\"${INSTALL_DIR}:\$PATH\""
    ;;
esac

echo ""
echo "[mimo-tui] next step:"
echo "             ${binname} init     # configure your MiMo API key"
echo "             ${binname}          # enter the REPL"
echo ""

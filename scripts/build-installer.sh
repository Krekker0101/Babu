#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required to build the installer" >&2
  exit 1
fi

if ! command -v npm >/dev/null 2>&1; then
  echo "npm is required to build the frontend" >&2
  exit 1
fi

if ! cargo tauri --version >/dev/null 2>&1; then
  echo "cargo-tauri is required. Install it with: cargo install tauri-cli --version '^2'" >&2
  exit 1
fi

if [ -f frontend/package-lock.json ]; then
  npm --prefix frontend ci
else
  npm --prefix frontend install
fi

npm --prefix frontend run build
cargo build -p babu-app --release
python post_build.py --sync

(
  cd crates/babu-gui
  cargo tauri build
)

cat <<MSG
Installer build finished.
Check Tauri bundle artifacts under:
  ${CARGO_TARGET_DIR:-$ROOT_DIR/target}/release/bundle
MSG

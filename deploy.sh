#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

BINARY_PATH="src-tauri/target/release/compressa_studio"
RECOMPILE_CHOICE=""

usage() {
    echo "Usage: $0 [-y|-n]"
    echo "  -y  recompile before running"
    echo "  -n  run the existing binary without recompiling"
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        -y)
            RECOMPILE_CHOICE="y"
            ;;
        -n)
            RECOMPILE_CHOICE="n"
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            usage >&2
            exit 1
            ;;
    esac
    shift
done

build_binary() {
    if [ ! -d "node_modules" ]; then
        echo "Installing npm dependencies..."
        npm install
    fi

    npm run tauri build -- --no-bundle
}

if [ ! -f "$BINARY_PATH" ]; then
    echo "Compiled binary not found at $BINARY_PATH. Initiating build process..."
    build_binary
else
    if [ -z "$RECOMPILE_CHOICE" ]; then
        read -r -p "Pre-compiled binary found. Recompile before running? [Y/n] " RECOMPILE_CHOICE
        RECOMPILE_CHOICE="${RECOMPILE_CHOICE:-y}"
    fi

    case "${RECOMPILE_CHOICE,,}" in
        y|yes)
            echo "Recompiling before launch..."
            build_binary
            ;;
        n|no)
            echo "Pre-compiled binary found. Skipping compilation."
            ;;
        *)
            echo "Invalid choice: $RECOMPILE_CHOICE" >&2
            usage >&2
            exit 1
            ;;
    esac
fi

echo "Executing $BINARY_PATH..."
exec "$BINARY_PATH"

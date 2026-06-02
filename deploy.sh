#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

BINARY_PATH="src-tauri/target/release/compressa_studio"

if [ ! -f "$BINARY_PATH" ]; then
    echo "Compiled binary not found at $BINARY_PATH. Initiating build process..."
    
    if [ ! -d "node_modules" ]; then
        echo "Installing npm dependencies..."
        npm install
    fi
    
    npm run tauri build -- --no-bundle
else
    echo "Pre-compiled binary found. Skipping compilation."
fi

echo "Executing $BINARY_PATH..."
exec "$BINARY_PATH"

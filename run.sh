#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

LOCAL_URL="http://localhost:5173"

echo "Compressa Studio"
echo "Local web address: $LOCAL_URL"
echo "Starting the Tauri desktop app. Keep this terminal open while using it."
echo

if [ ! -d "node_modules" ]; then
  echo "Installing npm dependencies first..."
  npm install
  echo
fi

npm run tauri -- dev

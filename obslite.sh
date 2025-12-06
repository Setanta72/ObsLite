#!/bin/bash
# ObsLite Launcher
# Includes WebKit fixes for Raspberry Pi / ARM Linux

export WEBKIT_DISABLE_DMABUF_RENDERER=1
export WEBKIT_DISABLE_COMPOSITING_MODE=1

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Try to find the binary
if [ -f "$SCRIPT_DIR/src-tauri/target/release/obslite" ]; then
    exec "$SCRIPT_DIR/src-tauri/target/release/obslite" "$@"
elif [ -f "/usr/bin/obslite" ]; then
    exec /usr/bin/obslite "$@"
elif [ -f "$HOME/.local/bin/obslite" ]; then
    exec "$HOME/.local/bin/obslite" "$@"
else
    echo "ObsLite binary not found"
    exit 1
fi

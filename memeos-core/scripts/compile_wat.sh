#!/usr/bin/env bash
set -euo pipefail
# Compile tests/hello.wat -> tests/hello.wasm using wat2wasm if available
if command -v wat2wasm >/dev/null 2>&1; then
  wat2wasm tests/hello.wat -o tests/hello.wasm
  echo "Compiled tests/hello.wasm"
else
  echo "wat2wasm not found. Install wabt or run 'cargo install wat' to compile WAT to WASM." >&2
  exit 1
fi

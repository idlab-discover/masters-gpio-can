#!/bin/bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
REMOTE_DIR=~/can

artifacts=(
  target/aarch64-unknown-linux-gnu/release/echo
  target/aarch64-unknown-linux-gnu/release/native
  target/wasm32-wasip2/release/wasm.wasm
)

pids=()

for host in "$@"; do
  for artifact in "${artifacts[@]}"; do
    scp "$SCRIPT_DIR/$artifact" "$host:$REMOTE_DIR/${artifact##*/}" &
    pids+=("$!")
  done
done

status=0

for pid in "${pids[@]}"; do
  wait "$pid" || status=1
done

exit "$status"

#!/bin/bash
set -euo pipefail

REMOTE_DIR=~/can/blocking

artifacts=(
  target/aarch64-unknown-linux-gnu/release/native-recv
  target/aarch64-unknown-linux-gnu/release/native-send
  target/wasm32-wasip2/release/wasm_recv.wasm
  target/wasm32-wasip2/release/wasm_send.wasm
)

pids=()

for host in "$@"; do
  for artifact in "${artifacts[@]}"; do
    scp "$artifact" "$host:$REMOTE_DIR" &
    pids+=("$!")
  done
done

status=0

for pid in "${pids[@]}"; do
  wait "$pid" || status=1
done

exit "$status"

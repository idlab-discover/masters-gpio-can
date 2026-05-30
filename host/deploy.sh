#!/bin/bash
set -euo pipefail

REMOTE_DIR=~/can/

artifacts=(
  target/aarch64-unknown-linux-gnu/release/host
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

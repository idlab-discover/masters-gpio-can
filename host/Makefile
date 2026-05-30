.PHONY: all

HOST_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
REPO_DIR := $(abspath $(HOST_DIR)/..)

all:
	cd $(REPO_DIR) && CARGO_TARGET_DIR=host/target cross build --manifest-path host/Cargo.toml --target=aarch64-unknown-linux-gnu --release

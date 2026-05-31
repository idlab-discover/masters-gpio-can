.PHONY: all

MESSAGE_DIR := $(patsubst %/,%,$(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

all:
	cd $(MESSAGE_DIR) && cross build --target=aarch64-unknown-linux-gnu --release -p blocking-native-recv -p blocking-native-send -p nb-native-recv -p nb-native-send
	cd $(MESSAGE_DIR) && cargo build --target=wasm32-wasip2 --release -p blocking-wasm-recv -p blocking-wasm-send -p nb-wasm-recv -p nb-wasm-send

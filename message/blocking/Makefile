.PHONY: all

all:
	cross build --target=aarch64-unknown-linux-gnu --release -p native-recv -p native-send
	cargo build --target=wasm32-wasip2 --release -p wasm-recv -p wasm-send

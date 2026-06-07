.PHONY: all

all:
	cross build --target=aarch64-unknown-linux-gnu --release -p echo -p native
	cargo build --target=wasm32-wasip2 --release -p wasm
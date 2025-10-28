.PHONY: build

build:
	cargo build --release --bin erebus-server
	cargo build --release --bin erebus-server-cli
	mkdir -p "build"
	mkdir -p "build/data"
	cp target/release/erebus-server build/server
	cp target/release/erebus-server-cli build/cli
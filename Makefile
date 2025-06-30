BINARY_NAME := rtask

.PHONY: all build install uninstall clippy check clean

all: build

build:
	cargo build --release

install:
	cargo install --path .

uninstall:
	cargo uninstall $(BINARY_NAME)

clippy:
	cargo clippy -- -D warnings

check: clippy
	cargo check --release

clean:
	cargo clean

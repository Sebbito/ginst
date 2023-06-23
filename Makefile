all: lint build test

lint:
	cargo clippy -- -D warnings

build:
	cargo build

test:
	cargo check
	cargo test

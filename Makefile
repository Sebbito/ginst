
test:
	cargo test

build:
	cargo build

clean:
	cargo clean

# make release builds for all architectures
release: clean
	cross b -r --target aarch64-unknown-linux-gnu
	cross b -r --target aarch64-unknown-linux-musl
	cross b -r --target x86_64-unknown-linux-gnu
	cross b -r --target x86_64-unknown-linux-musl
	

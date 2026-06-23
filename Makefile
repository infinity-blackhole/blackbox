.PHONY: build test check lint coverage run-dev clean

build:
	cargo build --workspace

test:
	cargo test --workspace

check:
	cargo check --workspace
	cargo clippy --workspace -- -D warnings

lint:
	typos
	nix fmt

coverage:
	cargo tarpaulin --workspace --out Html

run-dev:
	cargo run -p blackbox-dev

clean:
	cargo clean

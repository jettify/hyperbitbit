test:
	cargo test --all-features -- --nocapture --test-threads 1

build:
	cargo test  --all-features

lint:
	cargo clean
	cargo check
	cargo clippy

run:
	cargo run --example simple

doc:
	cargo doc --open

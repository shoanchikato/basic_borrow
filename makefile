run:
	RUST_BACKTRACE=1 cargo run

fmt:
	cargo fmt

lint:
	cargo clippy

release:
	cargo build --release

run-r:
	cargo run --release
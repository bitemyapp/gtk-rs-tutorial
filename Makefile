build:
	cargo build

run: build
	RUST_BACKTRACE=1 ./target/debug/gtk-rs-tutorial

build-watch:
	cargo watch -x build

test:
	cargo test -- --nocapture

test-debug:
	RUST_BACKTRACE=1 RUST_LOG=gtk-rs-tutorial=debug cargo test -- --nocapture

fmt:
	cargo fmt

rustfix:
	rustfix

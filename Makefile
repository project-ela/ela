build:
	cargo build

run:
	cargo run

test:
	cargo test -- --nocapture

.PHONY: build run test

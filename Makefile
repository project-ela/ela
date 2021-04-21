build:
	cargo build

run:
	cargo run

test:
	cargo test -- --nocapture --test-threads 1

.PHONY: build run test

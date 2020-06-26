build:
	cargo build

run:
	cargo run -- ${FILE}

test:
	cargo test

.PHONY: build run test

build:
	cargo build --verbose

run:
	cargo run -- ${FILE}

test:
	cargo test --verbose
	cd sigrun && ./test.sh

.PHONY: build run test

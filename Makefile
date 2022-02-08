CARGO=cargo

.PHONY: build
build:
	${CARGO} build

.PHONY: build
test:
	${CARGO} test
	cd sigrun && ./test.sh

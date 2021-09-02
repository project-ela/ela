build:
	cargo build --verbose

run:
	cargo run -- ${FILE}

test:
	cargo test --verbose
	cd sigrun && ./test.sh

run-example: FILE = helloworld.vd
run-example:
	@echo "==> Compiling..."
	@cat ./examples/stdlib.vd ./examples/${FILE} > ./tmp.vd
	@cargo run -q -p sigrun -- ./tmp.vd ./tmp.s

	@echo "==> Assembling..."
	@cargo run -q -p rota -- ./tmp.s ./tmp.o
	@cargo run -q -p rota -- ./examples/crt0.s ./tmp_crt0.o

	@echo "==> Linking..."
	@cargo run -q -p herja -- ./tmp_crt0.o ./tmp.o  ./tmp

	@echo "==> Emulating..."
	@cargo run -q -p eir -- ./tmp

.PHONY: build run test

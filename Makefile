test:
	cargo test -- --show-output --test-threads=4

run:
	cargo run --bin main

build:
	cargo build

fmt:
	cargo fmt

watch:
ifdef OS
	watch.cmd main
else
	sudo sh watch.sh main
endif
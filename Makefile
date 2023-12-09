test:
	cargo test -- --show-output --test-threads=4

run:
	cargo run --bin main -- $(ARGS)

build:
	cargo build

fmt:
	cargo fmt

watch:
ifdef OS
	watch.cmd main -- $(ARGS)
else
	sudo sh watch.sh main -- $(ARGS)
endif
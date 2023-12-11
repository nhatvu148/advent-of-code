test:
	cargo test -- --show-output --test-threads=4

run:
	cargo run --bin main -- $(f)

build:
	cargo build

fmt:
	cargo fmt

watch:
ifdef OS
	watch.cmd main $(f)
else
	sudo sh watch.sh main $(f)
endif
fmt:
	cargo fmt

clippy:
	cargo clippy

test:
	cargo test

build:
	cargo build

run:
	cargo run parse config.ini

all: fmt clippy test build
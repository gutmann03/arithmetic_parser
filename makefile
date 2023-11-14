prog :=arithmetic_parser_smetaniuk

debug ?=

file ?=

ifdef debug
	release :=
	target :=debug
	extension :=-debug
else
	release :=--release
	target :=release
	extension :=
endif

test:
	cargo test

clippy:
	cargo clippy

fmt:
	cargo fmt

pretty:
	cargo fmt
	cargo clippy
	cargo test

build:
	cargo build $(release)

run_console:
	target/$(target)/$(prog) -c

run_file:
	target/$(target)/$(prog) -f $(file)

all: build install

help:
	@echo "usage: make $(prog) [debug=1] [file=<file_name>]"
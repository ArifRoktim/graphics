# path to the script file to run
S := scripts/torus_simple.mdl

.PHONY: all build dev debug clear clean wipe

all:
	cargo run --release $(S)
	@./.animate.sh $(S)

build:
	cargo build --release

dev:
	cargo run $(S)
	@./.animate.sh $(S)

debug:
	RUST_BACKTRACE=1 cargo run $(S)

test:
	RUST_BACKTRACE=1 cargo test

clear:
	rm -vrf *.bak out/*

clean:
	./.clean.sh

wipe: clear clean

# path to the script file to run
S := scripts/solids

.PHONY: all dev debug clear clean check test clippy fmt

all:
	cargo run --release $(S)

dev:
	cargo run $(S)

debug:
	RUST_BACKTRACE=1 cargo run $(S)

clear:
	-rm *.ppm *.png *.bak

clean: clear
	cargo clean

check test:
	@touch src/main.rs
	cargo $@

clippy fmt:
	@touch src/main.rs
	cargo +nightly $@

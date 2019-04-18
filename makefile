SCRIPT = scripts/solids

.PHONY: all dev debug clean clippy fmt

all:
	cargo run --release $(SCRIPT)

dev debug:
	cargo run $(SCRIPT)

clean:
	-rm *.ppm *.png *.bak
	cargo clean

check:
	@touch src/main.rs
	cargo check

clippy fmt:
	@touch src/main.rs
	cargo +nightly $@

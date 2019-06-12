# path to the script file to run
S := scripts/torus_simple.mdl

.PHONY: all dev debug clear clean

all:
	cargo run --release $(S)
	@./.animate.sh $(S)

dev:
	cargo run $(S)
	@./.animate.sh $(S)

debug:
	RUST_BACKTRACE=1 cargo run $(S)

clear:
	rm -vrf *.bak out/*

clean: clear
	cargo -v clean

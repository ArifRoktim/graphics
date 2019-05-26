# path to the script file to run
S := scripts/simple_anim.mdl

.PHONY: all dev debug clear clean

all:
	cargo run --release $(S)

dev:
	cargo run $(S)

debug:
	RUST_BACKTRACE=1 cargo run $(S)

clear:
	rm -vf *.bak out/*

clean: clear
	cargo -v clean

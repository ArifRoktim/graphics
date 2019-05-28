# path to the script file to run
S := scripts/simple_anim.mdl

.PHONY: all dev debug clear clean

all:
	cargo run --release $(S)
	animate -delay 1.7 out/simple_50.gif

dev:
	cargo run $(S)

debug:
	RUST_BACKTRACE=1 cargo run $(S)

clear:
	rm -vrf *.bak out/*

clean: clear
	cargo -v clean

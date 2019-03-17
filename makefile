.PHONY: all
all: src/main.rs
	cargo run scripts/curves

.PHONY: clean
clean:
	-rm *.ppm *.png *.bak
	cargo clean

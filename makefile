.PHONY: all
all: src/main.rs
	cargo run script

.PHONY: clean
clean:
	-rm *.ppm *.png

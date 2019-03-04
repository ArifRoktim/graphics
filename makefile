.PHONY: all
all: src/main.rs
	cargo run
	display out.ppm
	convert out.ppm out.png

.PHONY: clean
clean:
	-rm out.ppm out.png

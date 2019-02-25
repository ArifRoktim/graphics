.PHONY: all
all: src/main.rs
	cargo run
	echo "why not nightmare fuel?"
	display out.ppm
	convert out.ppm out.png

.PHONY: clean
clean:
	-rm out.ppm out.png

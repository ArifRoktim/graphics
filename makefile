.PHONY: all
all: src/main.rs
	cargo run scripts/cubism

.PHONY: clean
clean:
	-rm *.ppm *.png *.bak

.PHONY: all
all: src/main.rs
	cargo run --release scripts/polygons

.PHONY: debug
dev: src/main.rs
	cargo run scripts/polygons

.PHONY: clean
clean:
	-rm *.ppm *.png *.bak
	cargo clean

.PHONY: clippy
clippy:
	touch src/main.rs
	cargo clippy

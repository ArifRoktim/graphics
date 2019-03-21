.PHONY: all
all: src/main.rs
	cargo run --release scripts/3d

.PHONY: debug
debug: src/main.rs
	cargo run scripts/3d

.PHONY: clean
clean:
	-rm *.ppm *.png *.bak
	cargo clean

.PHONY: clippy
clippy:
	touch src/main.rs
	cargo clippy

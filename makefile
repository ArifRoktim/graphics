SCRIPT = scripts/cstack

.PHONY: all
all: src/main.rs
	cargo run --release $(SCRIPT)

.PHONY: dev
dev: src/main.rs
	cargo run $(SCRIPT)

.PHONY: clean
clean:
	-rm *.ppm *.png *.bak
	cargo clean

.PHONY: clippy
clippy:
	touch src/main.rs
	cargo clippy

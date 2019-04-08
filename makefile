SCRIPT = scripts/cstack

.PHONY: all
all: src/main.rs
	cargo run --release $(SCRIPT)
	@echo -e 'NOTE:\ni know it dont work very much good right now. :(\ni''ll fix it tomorrow...'

.PHONY: debug
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

all: src/main.rs
	cargo run
	display out.ppm
	convert out.ppm out.png

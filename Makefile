all:
	cargo build

run:
	cargo run -- ./res/models/42.obj

clean:
	cargo clean

.PHONY: all clean run
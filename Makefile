
all: README.md

README.md: src/lib.rs
	cargo readme > $@

test:
	cargo test -p test-helper

clean:
	cargo clean

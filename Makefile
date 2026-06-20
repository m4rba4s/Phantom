.PHONY: all check test lint build clean audit

all: check lint test build

check:
	cargo check --all-features

test:
	cargo test --all-features

lint:
	cargo clippy -- -D warnings
	python3 -m ruff check scripts/offensive/ || true

audit:
	cargo audit || true

build:
	cargo build --release

clean:
	cargo clean

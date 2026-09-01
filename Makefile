.PHONY: build test bench fmt clippy clean

build:
	source ~/.cargo/env && cargo build --workspace

test:
	source ~/.cargo/env && cargo test --workspace

bench:
	source ~/.cargo/env && cargo bench --workspace

fmt:
	source ~/.cargo/env && cargo fmt --all

clippy:
	source ~/.cargo/env && cargo clippy --workspace

clean:
	source ~/.cargo/env && cargo clean

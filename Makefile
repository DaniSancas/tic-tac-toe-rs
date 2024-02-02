RUST_BIN := tic-tac-toe-rs
CARGO_CHECK := @command -v cargo >/dev/null 2>&1 || { echo >&2 "Cargo is not installed. Please install Rust from https://www.rust-lang.org/tools/install. Aborting."; exit 1; }

.PHONY: install uninstall build clean run help

help:
	@echo "Available targets:"
	@echo "  install     : Install the Rust binary locally"
	@echo "  uninstall   : Uninstall the locally installed Rust binary"
	@echo "  build       : Build the Rust binary in release mode"
	@echo "  clean       : Remove build artifacts"
	@echo "  run         : Run the installed or built Rust binary"
	@echo "  help        : Show this help message"

install:
	$(CARGO_CHECK)
	cargo install --path .

uninstall:
	@echo "Uninstalling $(RUST_BIN)"
	@cargo uninstall $(RUST_BIN)

build:
	$(CARGO_CHECK)
	cargo build --release

clean:
	$(CARGO_CHECK)
	@echo "Cleaning build artifacts"
	@cargo clean

run:
	$(CARGO_CHECK)
	@if command -v $(RUST_BIN) >/dev/null 2>&1; then \
		$(RUST_BIN); \
	elif test -f target/release/$(RUST_BIN); then \
		./target/release/$(RUST_BIN); \
	else \
		echo "No binary found. Please run 'make install' or 'make build'."; \
	fi

# Freshfetch Makefile
# Convenience targets for building, testing, and installing

PREFIX ?= /usr/local
BINARY := freshfetch

.PHONY: all build release install uninstall clean test check pkg

# Default target
all: release

# Development build
build:
	@echo "Building (debug)..."
	@cargo build
	@echo "Done."

# Release build
release:
	@echo "Building (release)..."
	@cargo build --release
	@echo "Done."

# Run tests
test:
	@echo "Running tests..."
	@cargo test
	@echo "Done."

# Run clippy and check
check:
	@echo "Running checks..."
	@cargo clippy -- -D warnings
	@cargo fmt --check
	@echo "Done."

# Install to system
install: release
	@echo "Installing to $(PREFIX)/bin..."
	@install -Dm755 target/release/$(BINARY) $(PREFIX)/bin/$(BINARY)
	@echo "Installed."

# Uninstall from system
uninstall:
	@echo "Removing $(PREFIX)/bin/$(BINARY)..."
	@rm -f $(PREFIX)/bin/$(BINARY)
	@echo "Removed."

# Clean build artifacts
clean:
	@echo "Cleaning..."
	@cargo clean
	@rm -rf pkg/ *.tar.gz
	@echo "Done."

# Create distributable package
pkg: release
	@echo "Packaging..."
	@mkdir -p ./pkg/usr/bin
	@install -Dm755 target/release/$(BINARY) ./pkg/usr/bin/$(BINARY)
	@tar -zcvf $(BINARY).tar.gz -C pkg .
	@echo "Package created: $(BINARY).tar.gz"

# Run the application
run:
	@cargo run

# Help
help:
	@echo "Freshfetch Makefile targets:"
	@echo "  all       - Build release binary (default)"
	@echo "  build     - Build debug binary"
	@echo "  release   - Build release binary"
	@echo "  test      - Run tests"
	@echo "  check     - Run clippy and format check"
	@echo "  install   - Install to $(PREFIX)/bin"
	@echo "  uninstall - Remove from $(PREFIX)/bin"
	@echo "  clean     - Remove build artifacts"
	@echo "  pkg       - Create distributable tarball"
	@echo "  run       - Run in development mode"
	@echo "  help      - Show this help"

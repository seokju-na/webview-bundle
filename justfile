_default:
  just --list -u

alias t := test
alias f := format
alias l := lint
alias b := build
alias x := xtask

# Setup development environment
setup:
  # Install Rust-related tools
  cargo install cargo-binstall
  cargo binstall taplo-cli

  # Setup Node.js environment
  corepack enable
  corepack prepare --activate
  yarn

# Test all files
test: test-rust test-js

# Test JS files
test-js: build
  yarn vitest run

# Test Rust files
test-rust:
  cargo test --workspace --no-fail-fast

# Format all files
format: format-toml format-rust format-js

# Format TOML files
format-toml:
  taplo format

# Format Rust files
format-rust:
  cargo fmt --all

# Format JS files via Biome
format-js:
  yarn biome format

# Lint all files
lint: lint-rust lint-js

# Lint JS files via Biome
lint-js:
  yarn biome check

# Lint Rust files via Clippy
lint-rust:
  cargo clippy --workspace

# Typechecking with TSC
typecheck:
  tsc --noEmit

# Build as release mode
build:
  yarn workspaces foreach -Apt run build

# Build as debug mode
build-debug:
  yarn workspaces foreach -Apt run build:debug

# Run benchmarks
benchmark: build
  yarn workspaces foreach -A --include='@benchmark/*' run bench

# Run xtask
xtask *ARGS:
  yarn xtask {{ARGS}}

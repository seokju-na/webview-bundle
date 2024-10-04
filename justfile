_default:
  just --list -u

alias t := test
alias f := format
alias l := lint
alias b := build

# Setup development environment
setup:
  # Install Rust-related tools
  cargo install cargo-binstall
  cargo binstall taplo-cli knope

  # Setup Node.js environment
  corepack enable
  corepack prepare --activate
  yarn

test: test-rust test-js

test-js: build-debug
  yarn vitest run

test-rust:
  cargo test --workspace --no-fail-fast

biome:
  yarn biome check

format: format-toml format-rust format-js

format-toml:
  taplo format

format-rust:
  cargo fmt --all

format-js:
  yarn biome format

lint: lint-rust lint-js

lint-js:
  yarn biome check

lint-rust:
  cargo clippy --workspace

build:
  yarn workspaces foreach -Apt run build

build-debug:
  yarn workspaces foreach -Apt run build:debug

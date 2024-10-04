_default:
  just --list -u

alias t := test
alias tr := test-rust
alias f := format
alias l := lint
alias lr := lint-rust
alias b := build

# Installs the tools needed to develop
install-tools:
	cargo install cargo-binstall
	cargo binstall taplo-cli knope

test:
  yarn vitest run

test-rust:
  cargo test --workspace

biome:
  yarn biome check

format:
  cargo fmt
  taplo format

lint:
  yarn biome check

lint-rust:
  cargo clippy

build:
  yarn workspaces foreach -Apt run build

build-debug:
  yarn workspaces foreach -Apt run build:debug

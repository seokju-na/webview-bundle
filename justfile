_default:
  just --list -u

alias t := test

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

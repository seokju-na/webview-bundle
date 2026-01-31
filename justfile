_default:
    just --list -u

alias t := test
alias f := format
alias l := lint
alias b := build
alias x := xtask
alias tsc := typecheck
alias bench := benchmark

# Setup development environment
setup:
    # Setup Node.js environment
    corepack enable
    corepack prepare --activate
    yarn
    yarn lefthook install

    # Run build
    just build

# Test all files
test: test-rs test-js

# Test JS files
test-js: build-napi build-js
    yarn vitest run

# Test Rust files
test-rs:
    cargo test --workspace --no-fail-fast --all-features

# Format all files
format: format-toml format-rs format-js

# Format TOML files
format-toml:
    yarn taplo format

# Format Rust files
format-rs:
    cargo fmt --all

# Format JS files via Biome
format-js:
    yarn biome format --write

# Lint all files
lint: lint-rs lint-js

# Lint JS files via Biome
lint-js:
    yarn biome check

# Lint Rust files via Clippy
lint-rs:
    cargo clippy --workspace

# Typechecking with TSC
typecheck:
    yarn workspaces foreach -Apt run typecheck

# Build as release mode
build: build-rs build-napi build-js

# Build NAPI modules
build-napi:
    yarn workspaces foreach -Ap run build-napi

# Build Rust workspaces
build-rs:
    cargo build --workspace

# Build JS packages
build-js:
    yarn workspaces foreach -Apt run build

# Run benchmarks
benchmark: build
    yarn workspaces foreach -A --include='@benchmark/*' run bench

# Start website dev server
website:
    yarn workspace wvb-website run typegen
    yarn workspace wvb-website run dev

# Run xtask
xtask *ARGS:
    ./xtask/cli.ts {{ ARGS }}

# Prerelease
prerelease:
    git tag -a prerelease -m "prerelease" --force
    git push origin prerelease --force

# Release
release:
    git tag -a release -m "release" --force
    git push origin release --force

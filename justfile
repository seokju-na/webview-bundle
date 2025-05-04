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

    # Pre-requirements
    yarn lefthook install
    yarn workspace xtask run out

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
    yarn taplo format

# Format Rust files
format-rust:
    cargo fmt --all

# Format JS files via Biome
format-js:
    yarn biome format --write

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
    ./xtask-ts/cli.ts {{ ARGS }}

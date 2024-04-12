# Read up on just here: https://github.com/casey/just

set fallback := true
set shell := ["bash", "-uc"]
set windows-shell := ["sh", "-uc"]

# `just --list` (or just `just`) will print all the recipes in the current
# Justfile. `just RECIPE` will run the macro/job.
_default:
  @just --list

# Install required packages.
install:
  pnpm install

# Builds everything (basically `cargo build`).
build:
  cargo build

# Test everything (basically `cargo test`).
test:
  cargo test

# Typically doesn't need to be run.
# Format in editor/on commit should do this automatically.
format:
  cargo fmt
  pnpm format

# Checks formatting for ci.
format-ci:
  cargo fmt --check
  pnpm format:check

# Lints everything (basically `cargo clippy`).
lint:
  cargo clippy -- -D warnings

# Dry run publish to cargo.
publish *args:
  cargo publish --dry-run {{ args }}

# Publish to cargo (basically `cargo publish`).
publish-push *args:
  cargo publish {{ args }}

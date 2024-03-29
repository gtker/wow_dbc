#!/usr/bin/env bash
set -e

export RUSTFLAGS="-D warnings"
export CARGO_INCREMENTAL=0

cargo install cargo-hack --locked

cargo test --all-features -p $1

# Try to catch clippy lints early
cargo clippy --all-features -p $1

cargo hack clippy --feature-powerset -p $1


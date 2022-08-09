#!/bin/bash

rustup default nightly
cargo install grcov
rustup component add llvm-tools-preview
export RUSTFLAGS="-C instrument-coverage"
cargo build --quiet

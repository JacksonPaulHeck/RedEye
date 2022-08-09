#!/bin/bash

rustup default nightly
cargo install grcov
rustup component add llvm-tools-preview
export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="./target/debug/coverage/default.profraw"
#cargo test  -- --nocapture
cargo test
grcov . --binary-path ./target/debug/deps -s . -t html --branch --ignore-not-existing -o ./target/debug/coverage/
for f in target/debug/coverage/*
do
    cp -r $f artifacts/
done

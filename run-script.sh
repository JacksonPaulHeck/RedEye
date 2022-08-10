#!/bin/bash

cargo build --verbose
cargo run --verbose examples/test/test.night -i
rm -rf target/debug/deps/*

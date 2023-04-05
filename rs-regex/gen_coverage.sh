#!/bin/bash

export RUSTFLAGS="-Cinstrument-coverage"
cargo build
export LLVM_PROFILE_FILE="cov-%p-%m.profraw"
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/
echo "coverage report generated"

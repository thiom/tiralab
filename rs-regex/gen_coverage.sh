#!/bin/bash

rm -rf target/
rm -r coverage/
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="cov-%p-%m.profraw"
cargo build
cargo test
grcov . -s . \
--binary-path ./target/debug/ \
-t html \
--ignore-not-existing \
-o ./coverage/ \
--filter covered \
--keep-only "src/*" \
--ignore "src/main.rs" \
--ignore "src/lib.rs" \
--ignore "src/tokens.rs" \
--ignore "src/ast.rs" \
--ignore "*.cargo/*" \
--excl-start "grcov-excl-start" \
--excl-stop "grcov-excl-stop" \
--excl-line "grcov-excl-line|#\\[derive\\(|//!"

rm *.profraw
echo "coverage report generated"

#!/bin/sh
cargo install cargo-fuzz
# cd fuzz
# cargo clean eth_pairings_go
# cargo update
# cd ..
cargo +nightly fuzz run --release fuzz_target_compare_ops -- -max_len=16000 -jobs=1000 -workers=32
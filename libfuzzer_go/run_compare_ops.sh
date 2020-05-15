#!/bin/sh
cargo install cargo-fuzz
cd fuzz
cargo update
cd ..
cargo +nightly fuzz run --release fuzz_target_compare_ops -- -max_len=16000 -print_pcs=1 -print_final_stats=1 -jobs=1000 
#-workers=32
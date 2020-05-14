# Run

```
cd libfuzzer_go
cargo install cargo-fuzz
cargo +nightly fuzz run --release fuzz_target_compare_ops -- -max_len=16000 -jobs=1000
```

# Stop

```
Ctrl-C
./kill_all_childs.sh
```
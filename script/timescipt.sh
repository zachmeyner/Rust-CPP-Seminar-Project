# !/bin/bash



cd ../rust/extended_seive
cargo build --release

START=$(date +%s.%N)
cargo run --release 100000000
DUR=$(echo "$(date +%s.%N) - $START" | bc)

echo "Took $DUR seconds"


cd ../../script
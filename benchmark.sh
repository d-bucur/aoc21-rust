#!/bin/bash
cargo build --release

format_table() {
  sed -i 's/`.*release\/day\([0-9]\+\) \([0-9]\)`/[day \1 part \2](\/src\/bin\/day\1.rs)/g' results.md
}

hyperfine_bench() {
  if [ "$1" = "all" ]; then
    hyperfine\
  --warmup 3\
  -P day 1 14\
  "./target/release/aoc {day} 1"\
  "./target/release/aoc {day} 2"\
  --export-markdown results.md
  format_table
  else
    hyperfine --warmup 3 "./target/release/aoc ${1} ${2}"
  fi
}

if [ "$1" = "hyperfine" ]; then
  hyperfine_bench $2 $3
elif [ "$1" = "iai" ]; then
  cargo bench --bench iai_benchmarks
elif [ "$1" = "criterion" ]; then
  cargo bench --bench criterion_benchmarks
fi

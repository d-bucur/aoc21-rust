#!/bin/bash
cargo build --release

format_table() {
  sed -i 's/`.*release\/day\([0-9]\+\) \([0-9]\)`/[day \1 part \2](\/src\/bin\/day\1.rs)/g' results.md
}

if [ "$1" = "all" ]; then
  hyperfine\
 --warmup 3\
 -P day 1 13\
 "./target/release/aoc {day} 1"\
 "./target/release/aoc {day} 2"\
 --export-markdown results.md
 format_table
else
  hyperfine --warmup 3 "./target/release/aoc ${1} ${2}"
fi


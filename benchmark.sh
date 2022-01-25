#!/bin/bash
cargo build --release

format_table() {
  sed -i 's/`.*release\/day\([0-9]\) \([0-9]\)`/[day \1 part \2](\/src\/bin\/day\1.rs)/g' results.md
}

if [ "$1" = "all" ]; then
  hyperfine\
 --warmup 3\
 -P day 1 7\
 "cat inputs/day{day}.input | ./target/release/day{day} 1"\
 "cat inputs/day{day}.input | ./target/release/day{day} 2"\
 --export-markdown results.md
 format_table
else
  hyperfine --warmup 3 "cat inputs/day${1}.input | ./target/release/day${1} ${2}"
fi


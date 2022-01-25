#!/bin/bash
cargo build --release

if [ "$1" = "all" ]; then
  hyperfine\
 -P day 5 6\
 --warmup 3\
 "cat inputs/day{day}.input | ./target/release/day{day} 1"\
 -n "day{day} p1"\
 "cat inputs/day{day}.input | ./target/release/day{day} 2"\
 -n "day{day} p2"\
 --export-markdown results.md
else
  hyperfine --warmup 3 "cat inputs/day${1}.input | ./target/release/day${1} ${2}"
fi


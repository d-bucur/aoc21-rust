# How to run

```bash
cat inputs/day{}.input | cargo run --bin day{} -- {part}
```

Newer files use the same binary for part1 and part2 and the first parameter indicates which part to run. Example

```bash
cat inputs/day5.input | cargo run --bin day5 -- 2
```

Older files have different binaries for different parts 

```bash
cat inputs/day1.input | cargo run --bin day1p2
```


# How to run

```bash
cat inputs/day{}.input | cargo run --bin day{} -- {part}
```

Example

```bash
cat inputs/day5.input | cargo run --bin day5 -- 2
```

# Benchmark

[Results](results.md)

**Prerequisites**: install [hyperfine](https://github.com/sharkdp/hyperfine)

```
./benchmark all
```

To execute all benchmarks and create the markdown table

```
./benchmark {day} {part}
```

To benchmark only a specific solution

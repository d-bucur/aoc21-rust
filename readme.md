# How to run

```bash
cargo run --bin day{} -- {part}
```

Example

```bash
cargo run --bin day5 -- 2
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

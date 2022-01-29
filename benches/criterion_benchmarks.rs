use std::time::Duration;

use aoc::*;
use criterion::{criterion_group, criterion_main, Criterion};
use gag::Gag;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day13 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day13::part1()
        })
    });
    c.bench_function("day13 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day13::part2()
        })
    });
}

// criterion_group!(benches, criterion_benchmark);
criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(50).warm_up_time(Duration::from_secs(1)).measurement_time(Duration::from_secs(4));
    targets = criterion_benchmark
}
criterion_main!(benches);

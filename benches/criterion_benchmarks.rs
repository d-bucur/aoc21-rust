use std::time::Duration;

use aoc::*;
use criterion::{criterion_group, criterion_main, Criterion};
use gag::Gag;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day1::part1()
        })
    });

    c.bench_function("day1 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day1::part2()
        })
    });

    c.bench_function("day2 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day2::part1()
        })
    });

    c.bench_function("day2 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day2::part2()
        })
    });

    c.bench_function("day3 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day3::part1()
        })
    });

    c.bench_function("day3 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day3::part2()
        })
    });

    c.bench_function("day4 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day4::part1()
        })
    });

    c.bench_function("day4 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day4::part2()
        })
    });

    c.bench_function("day5 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day5::part1()
        })
    });

    c.bench_function("day5 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day5::part2()
        })
    });

    c.bench_function("day6 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day6::part1()
        })
    });

    c.bench_function("day6 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day6::part2()
        })
    });

    c.bench_function("day7 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day7::part1()
        })
    });

    c.bench_function("day7 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day7::part2()
        })
    });

    c.bench_function("day8 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day8::part1()
        })
    });

    c.bench_function("day8 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day8::part2()
        })
    });

    c.bench_function("day9 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day9::part1()
        })
    });

    c.bench_function("day9 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day9::part2()
        })
    });

    c.bench_function("day10 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day10::part1()
        })
    });

    c.bench_function("day10 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day10::part2()
        })
    });

    c.bench_function("day11 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day11::part1()
        })
    });

    c.bench_function("day11 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day11::part2()
        })
    });

    c.bench_function("day12 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day12::part1()
        })
    });

    c.bench_function("day12 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day12::part2()
        })
    });

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

    c.bench_function("day14 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day14::part1()
        })
    });

    c.bench_function("day14 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day14::part2()
        })
    });

    c.bench_function("day15 part1", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day15::part1()
        })
    });

    c.bench_function("day15 part2", |b| {
        b.iter(|| {
            let _gag = Gag::stdout().unwrap();
            day15::part2()
        })
    });
}

// criterion_group!(benches, criterion_benchmark);
criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20).warm_up_time(Duration::from_secs(1)).measurement_time(Duration::from_secs(5));
    targets = criterion_benchmark
}
criterion_main!(benches);

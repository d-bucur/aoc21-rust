type ResultFn = fn() -> Option<u64>;

use aoc::*;

fn main() {
    let solutions: Vec<[ResultFn; 2]> = vec![
        [day1::part1, day1::part2],
        [day2::part1, day2::part2],
        [day3::part1, day3::part2],
        [day4::part1, day4::part2],
        [day5::part1, day5::part2],
        [day6::part1, day6::part2],
        [day7::part1, day7::part2],
        [day8::part1, day8::part2],
        [day9::part1, day9::part2],
        [day10::part1, day10::part2],
        [day11::part1, day11::part2],
        [day12::part1, day12::part2],
        [day13::part1, day13::part2],
        [day14::part1, day14::part2],
    ];
    let day = std::env::args().nth(1).unwrap().parse::<usize>().unwrap() - 1;
    let part = std::env::args().nth(2).unwrap().parse::<usize>().unwrap() - 1;

    let solution_to_run = solutions[day][part];
    solution_to_run();
}

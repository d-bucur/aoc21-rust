use std::env;

use aoc::read_lines;

fn read_input() -> Vec<u32> {
    read_lines()
        .next()
        .unwrap()
        .split(',')
        .map(|e| e.parse::<u32>().unwrap())
        .collect()
}

pub fn part1() {
    let positions = read_input();
    let (&a, &b) = (
        positions.iter().min().unwrap(),
        positions.iter().max().unwrap(),
    );

    let dist_min = (a..b).map(|p| distance(&positions, p)).min().unwrap();
    println!("{dist_min}");

    // let avg = positions.iter().sum::<u32>() as f32 / positions.len() as f32;
    // println!("{avg}")
}

fn distance(positions: &[u32], t: u32) -> u32 {
    positions
        .iter()
        .map(|&p| (p as i32 - t as i32).abs() as u32)
        .sum::<u32>()
}

pub fn part2() {}

fn main() {
    let part = env::args().nth(1).unwrap();
    match part.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Invalid option"),
    }
}

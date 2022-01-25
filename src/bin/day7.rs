use std::env;
use rayon::prelude::*;

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
    let dist_min = (a..b).into_par_iter()
        .map(|p| distance_simple(&positions, p))
        .min()
        .unwrap();
    println!("{dist_min}");
}

fn distance_simple(positions: &[u32], t: u32) -> u32 {
    positions
        .iter()
        .map(|&p| (p as i32 - t as i32).abs() as u32)
        .sum::<u32>()
}

pub fn part2() {
    let positions = read_input();
    let (&a, &b) = (
        positions.iter().min().unwrap(),
        positions.iter().max().unwrap(),
    );
    let mut distances = vec![0u32; b as usize + 1];
    for i in 1..(b as usize + 1) {
        distances[i] = distances[i - 1] + i as u32;
    }

    let dist_min = (a..b).into_par_iter()
        .map(|p| distance_exp(&positions, p, &distances))
        .min()
        .unwrap();
    println!("{dist_min}");
}

fn distance_exp(positions: &[u32], t: u32, distances: &[u32]) -> u32 {
    positions
        .iter()
        .map(|&p| {
            let d = (p as i32 - t as i32).abs() as usize;
            distances[d]
        })
        .sum::<u32>()
}

fn main() {
    let part = env::args().nth(1).unwrap();
    match part.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Invalid option"),
    }
}

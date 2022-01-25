use std::env;

use aoc::read_lines;
use regex::Regex;

fn instruction_re() -> Regex {
    Regex::new(r"(\w+) (\d+)").unwrap()
}

pub fn part1() {
    let re = instruction_re();
    let mut horizontal = 0i32;
    let mut depth = 0i32;

    for line in read_lines() {
        let cap = re.captures(&line).unwrap();
        let dist = cap[2].parse::<i32>().unwrap();
        match &cap[1] {
            "forward" => horizontal += dist,
            "down" => depth += dist,
            "up" => depth -= dist,
            _ => (),
        };
    }

    println!("{}", horizontal * depth);
}

pub fn part2() {
    let re = instruction_re();
    let mut horizontal = 0i32;
    let mut depth = 0i32;
    let mut aim = 0i32;

    for line in read_lines() {
        let cap = re.captures(&line).unwrap();
        let val = cap[2].parse::<i32>().unwrap();
        match &cap[1] {
            "forward" => {
                horizontal += val;
                depth += aim * val;
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => (),
        };
    }

    println!("{}", horizontal * depth);
}

fn main() {
    let part = env::args().nth(1).unwrap();
    match part.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => println!("Invalid option"),
    }
}

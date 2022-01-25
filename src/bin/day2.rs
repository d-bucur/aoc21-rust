use std::env;

use aoc::read_lines;
use regex::Regex;

fn instruction_re() -> Regex {
    Regex::new(r"(\w+) (\d+)").unwrap()
}

pub fn part1() -> i32 {
    let re = instruction_re();
    let mut horizontal = 0i32;
    let mut depth = 0i32;

    for line in read_lines("2") {
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
    horizontal * depth
}

pub fn part2() -> i32 {
    let re = instruction_re();
    let mut horizontal = 0i32;
    let mut depth = 0i32;
    let mut aim = 0i32;

    for line in read_lines("2") {
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
    horizontal * depth
}

fn main() {
    let part = env::args().nth(1).unwrap();
    match part.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => {
            println!("Invalid option");
            0
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1488669, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1176514794, part2());
    }
}

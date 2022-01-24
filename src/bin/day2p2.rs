use regex::Regex;

mod common;
use crate::common::read_lines;

fn instruction_re() -> Regex {
    Regex::new(r"(\w+) (\d+)").unwrap()
}

pub fn main() {
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

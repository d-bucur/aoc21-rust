use regex::Regex;

use crate::common::read_lines;

pub fn part1() {
    let re = Regex::new(r"(\w+) (\d+)").unwrap();
    let mut horizontal = 0i32;
    let mut depth = 0i32;

    for line in read_lines() {
        let cap = re.captures(&line).unwrap();
        let dist = cap[2].parse::<i32>().unwrap();
        match cap[1].as_ref() {
            "forward" => horizontal += dist,
            "down" => depth += dist,
            "up" => depth -= dist,
            _ => (),
        };
    }

    println!("{}", horizontal * depth);
}
use aoc::read_lines;
use regex::Regex;

fn instruction_re() -> Regex {
    Regex::new(r"(\w+) (\d+)").unwrap()
}

pub fn main() {
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

use std::io::{self, BufRead, BufReader};

pub fn read_lines() -> impl Iterator<Item = String> {
    let stdin = io::stdin();
    let buffer = BufReader::new(stdin);
    let it = buffer.lines().map(|e| e.unwrap());
    it
}

#[allow(dead_code)]
fn main() {}

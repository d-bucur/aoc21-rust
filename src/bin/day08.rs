use std::collections::HashSet;

use aoc::read_lines;

/*
distinct
1 -> 2 segments
7 -> 3 segments
4 -> 4 segments
8 -> 7 segments
ambiguous
0,6,9 -> 6 segments
2,3,5 -> 5 segments
 */

fn part1() -> u32 {
    let known_lengths: HashSet<usize> = [2, 3, 4, 7].into_iter().collect();
    let mut total = 0;
    for line in read_lines("08") {
        let l: Vec<_> = line.split('|').collect();
        let outputs: Vec<&str> = l[1].split_whitespace().collect();
        for output in outputs {
            if known_lengths.contains(&output.len()) {
                total += 1;
            }
        }
    }
    println!("{total}");
    total
}

fn part2() -> u32 {
    0
}

fn main() {
    let part = std::env::args().nth(1).unwrap();
    match part.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => {
            println!("Invalid option");
            0u32
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(493, part1());
    }

    #[test]
    fn test_part2() {
        // assert_eq!(93397632, part2());
    }
}

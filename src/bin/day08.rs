use std::collections::{HashMap, HashSet};

use aoc::read_lines;
use itertools::Itertools;

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

/*
Logic to find the digits:
distinct:
1 -> 2 segments
7 -> 3 segments
4 -> 4 segments
8 -> 7 segments
ambiguous:
0,6,9 -> 6 segments
2,3,5 -> 5 segments
rules for ambigous:
6 segments:
6 intersect 1 = 1 segment
9 intersect 4 = 4 segment
0 remaining
5 segments:
3 intersect 1 = 2 segment
2 intersect 4 = 2 segment
5 remaining
*/

#[derive(Debug)]
struct SetCustom(Vec<bool>, u8, String); // TODO replace with ordered string/set of chars? remove u8?

fn part2() -> u32 {
    let mut total = 0;
    for line in read_lines("08") {
        let l: Vec<_> = line.split('|').collect();
        let digits: Vec<_> = l[0].split_whitespace().map(|e| to_set(e)).collect();
        let outputs: Vec<_> = l[1].split_whitespace().map(|e| to_set(e)).collect();

        let mut decyphered: [Option<&SetCustom>; 10] = Default::default();
        // TODO don't iterate over digits multiple times
        for d in digits.iter() {
            match d.1 {
                2 => decyphered[1] = Some(d),
                3 => decyphered[7] = Some(d),
                4 => decyphered[4] = Some(d),
                7 => decyphered[8] = Some(d),
                _ => (),
            }
        }
        let sixes = digits.iter().filter(|&s| s.1 == 6);
        for s in sixes {
            if intersect(s, decyphered[1].unwrap()) == 1 {
                decyphered[6] = Some(s);
            } else if intersect(s, decyphered[4].unwrap()) == 4 {
                decyphered[9] = Some(s);
            } else {
                decyphered[0] = Some(s);
            }
        }
        let fives = digits.iter().filter(|&s| s.1 == 5);
        for f in fives {
            if intersect(f, decyphered[1].unwrap()) == 2 {
                decyphered[3] = Some(f);
            } else if intersect(f, decyphered[4].unwrap()) == 2 {
                decyphered[2] = Some(f);
            } else {
                decyphered[5] = Some(f);
            }
        }

        let out_map: HashMap<String, usize> = decyphered
            .iter()
            .enumerate()
            .map(|(i, &s)| (s.unwrap().2.clone(), i))
            .collect();

        let mut num = 0;
        for (pos, set) in outputs.iter().enumerate() {
            let &digit = out_map.get(&set.2).unwrap();
            num += digit * 10usize.pow(3 - pos as u32);
        }
        println!("{} ", num);
        total += num;
    }
    println!("{}", total);
    total as u32
}

fn to_set(s: &str) -> SetCustom {
    let mut set = vec![false; 7];
    let a = 'a' as u8;
    let mut set_count = 0;
    for &c in s.as_bytes() {
        set[(c - a) as usize] = true;
        set_count += 1;
    }
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    SetCustom(set, set_count, String::from_iter(chars))
}

fn intersect(a: &SetCustom, b: &SetCustom) -> i32 {
    let mut common = 0;
    let SetCustom(a_mask, _, _) = a;
    let SetCustom(b_mask, _, _) = b;
    for (i, e) in a_mask.iter().enumerate() {
        if *e && b_mask[i] {
            common += 1;
        }
    }
    common
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
        assert_eq!(1010460, part2());
    }
}

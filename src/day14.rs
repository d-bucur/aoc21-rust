use std::collections::HashMap;

use regex::Regex;

use crate::{read_lines, Vec2d};

pub fn solve(steps: usize) -> Option<u64> {
    let re: Regex = Regex::new(r"(\w)(\w) -> (\w)").unwrap();
    let mut in_it = read_lines("14");
    let polymer_str = in_it.next().unwrap();
    in_it.next();

    let mut idx: usize = 0;
    let mut letter_to_idx = HashMap::new();
    let mut letters: Vec<char> = Default::default();
    let transforms_input: Vec<_> = in_it
        .map(|line| {
            let caps = re.captures(&line).unwrap();
            let left = caps[1].chars().next().unwrap();
            let right = caps[2].chars().next().unwrap();
            let result = caps[3].chars().next().unwrap();
            // println!("settings {}{}->{}", left, right, result);
            (
                create_idx(&mut letter_to_idx, left, &mut idx, &mut letters),
                create_idx(&mut letter_to_idx, right, &mut idx, &mut letters),
                create_idx(&mut letter_to_idx, result, &mut idx, &mut letters),
            )
        })
        .collect();
    // println!("{:?}", letter_to_idx);
    // println!("{:?}", letters);

    let mut transforms = Vec2d::new(idx, idx, 0);
    for (left, right, result) in transforms_input {
        transforms.set(left, right, result);
    }
    // println!("{:?}", transforms);

    let polymer: Vec<usize> = polymer_str
        .chars()
        .map(|c| *letter_to_idx.get(&c).unwrap())
        .collect();

    let mut count = vec![0u64; letters.len()];
    let mut pairs = HashMap::new();
    for i in 0..polymer.len() - 1 {
        let left = polymer[i];
        let right = polymer[i + 1];
        *pairs.entry((left, right)).or_insert(0) += 1;
        count[left] += 1;
    }
    count[*polymer.last().unwrap()] += 1;

    for _step in 0..steps {
        let mut new_pairs = HashMap::new();
        for ((left, right), pair_count) in pairs.iter() {
            let n = transforms.at(*left, *right);
            count[n] += pair_count;
            *new_pairs.entry((*left, n)).or_insert(0) += pair_count;
            *new_pairs.entry((n, *right)).or_insert(0) += pair_count;
        }
        pairs = new_pairs;
    }

    // println!("{pairs:?}");
    // println!("{count:?}");
    let min = *count.iter().min().unwrap();
    let max = *count.iter().max().unwrap();
    let res = max - min;
    println!("{res}");
    Some(res)
}

#[allow(dead_code)]
fn print_polymer(polymer: &Vec<usize>, letters: &Vec<char>) {
    for c in polymer.iter() {
        print!("{}", letters[*c]);
    }
    println!();
}

fn create_idx(
    letter_to_idx: &mut HashMap<char, usize>,
    val: char,
    idx: &mut usize,
    letters: &mut Vec<char>,
) -> usize {
    if let Some(old) = letter_to_idx.get(&val) {
        *old
    } else {
        letter_to_idx.insert(val, *idx);
        letters.push(val);
        *idx += 1;
        *idx - 1
    }
}

pub fn part1() -> Option<u64> {
    solve(10)
}

pub fn part2() -> Option<u64> {
    solve(40)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Some(2375), part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(Some(1976896901756), part2());
    }
}

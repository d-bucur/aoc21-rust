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
    let transforms_input: Vec<_> = in_it.map(|line| {
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
    }).collect();
    // println!("{:?}", letter_to_idx);
    // println!("{:?}", letters);

    let mut transforms = Vec2d::new(idx, idx, 0);
    for (left, right, result) in transforms_input {
        transforms.set(left, right, result);
    }
    // println!("{:?}", transforms);

    let mut polymer: Vec<usize> = polymer_str.chars().map(|c| *letter_to_idx.get(&c).unwrap()).collect();

    for _step in 0..steps {
        let mut next_polymer = Vec::<usize>::with_capacity(polymer.len() * 2 - 1);
        for i in 0..polymer.len()-1 {
            let left = polymer[i];
            let right = polymer[i+1];
            next_polymer.push(left);
            next_polymer.push(transforms.at(left, right));
            // print_polymer(&next_polymer, &letters);
        }
        next_polymer.push(polymer[polymer.len()-1]);
        polymer = next_polymer;
        // print_polymer(&polymer, &letters);
        println!("Finished step {_step}");
    }

    let mut counts = vec![0usize; letters.len()];
    for p in polymer {
        counts[p] += 1;
    }
    let min = *counts.iter().min().unwrap();
    let max = *counts.iter().max().unwrap();
    let res = (max - min) as u64;
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

fn create_idx(letter_to_idx: &mut HashMap<char, usize>, val: char, idx: &mut usize, letters: &mut Vec<char>) -> usize {
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
    // solve(40) // Too slow
    None
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
        // assert_eq!(Some(753), part2());
    }
}

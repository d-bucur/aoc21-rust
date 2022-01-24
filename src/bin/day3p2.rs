use std::collections::HashSet;

mod common;
use crate::common::read_lines;

pub fn main() {
    let length = 12;
    let mut all_nums: Vec<String> = Vec::new();
    let mut oxygen_nums = HashSet::new();
    let mut co2_nums = HashSet::new();

    for (i, l) in read_lines().enumerate() {
        all_nums.push(l);
        oxygen_nums.insert(i);
        co2_nums.insert(i);
    }

    let oxygen_idx = process(length, &all_nums, oxygen_nums, oxygen_filter);
    let oxygen = u32::from_str_radix(all_nums[oxygen_idx].as_str(), 2).unwrap();
    let co2_idx = process(length, &all_nums, co2_nums, co2_filter);
    let co2 = u32::from_str_radix(all_nums[co2_idx].as_str(), 2).unwrap();
    println!("{} = oxygen {:?} * co2 {:?}", oxygen * co2, oxygen, co2);
}

fn oxygen_filter(zeroes: &HashSet<usize>, ones: &HashSet<usize>) -> bool {
    zeroes.len() > ones.len()
}

fn co2_filter(zeroes: &HashSet<usize>, ones: &HashSet<usize>) -> bool {
    zeroes.len() <= ones.len()
}

fn process(
    length: usize,
    all_nums: &Vec<String>,
    mut valid_nums: HashSet<usize>,
    filter: fn(&HashSet<usize>, &HashSet<usize>) -> bool,
) -> usize {
    for i in 0usize..length {
        let (zeroes, ones) = divide(&all_nums, &valid_nums, i);
        if filter(&zeroes, &ones) {
            valid_nums = zeroes;
        } else {
            valid_nums = ones;
        }
        if valid_nums.len() == 1 {
            break;
        }
    }
    *valid_nums.iter().next().unwrap()
}

fn divide(
    all_nums: &Vec<String>,
    valid_nums: &HashSet<usize>,
    i: usize,
) -> (HashSet<usize>, HashSet<usize>) {
    let mut ones = HashSet::new();
    let mut zeroes = HashSet::new();
    for n in valid_nums {
        if all_nums[*n].as_bytes()[i] as char == '0' {
            zeroes.insert(*n);
        } else {
            ones.insert(*n);
        }
    }
    (zeroes, ones)
}

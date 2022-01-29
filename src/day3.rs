use std::collections::HashSet;

use crate::read_lines;

pub fn part1() -> Option<u64> {
    let length = 12;
    let mut total = 0u32;
    let mut ones = vec![0u32; length];

    for line in read_lines("3") {
        for (i, d) in line.chars().enumerate() {
            if d == '1' {
                ones[i] += 1;
            }
        }
        total += 1;
    }

    let half_total = total / 2;
    let mut gamma = 0u32;
    let mut epsilon = 0u32;
    for (i, v) in ones.iter().rev().enumerate() {
        let d = 2u32.pow(i as u32);
        if *v > half_total {
            gamma += d;
        } else {
            epsilon += d;
        }
    }
    println!("{} = gamma: {gamma} * epsilon: {epsilon}", gamma * epsilon);
    Some(gamma as u64 * epsilon as u64)
}

pub fn part2() -> Option<u64> {
    let length = 12;
    let mut all_nums: Vec<String> = Vec::new();
    let mut oxygen_nums = HashSet::new();
    let mut co2_nums = HashSet::new();

    for (i, l) in read_lines("3").enumerate() {
        all_nums.push(l);
        oxygen_nums.insert(i);
        co2_nums.insert(i);
    }

    let oxygen_idx = process(length, &all_nums, oxygen_nums, oxygen_filter);
    let oxygen = u32::from_str_radix(all_nums[oxygen_idx].as_str(), 2).unwrap();
    let co2_idx = process(length, &all_nums, co2_nums, co2_filter);
    let co2 = u32::from_str_radix(all_nums[co2_idx].as_str(), 2).unwrap();
    println!("{} = oxygen {:?} * co2 {:?}", oxygen * co2, oxygen, co2);
    Some(oxygen as u64 * co2 as u64)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(3009600, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(6940518, part2().unwrap());
    }
}

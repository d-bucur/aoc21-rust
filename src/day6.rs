use std::{collections::HashMap, thread};

use crate::read_lines;

const THREADS_MAX: usize = 8;

pub fn part1() -> Option<u64> {
    const DAYS: u32 = 80;
    // Keeping as example of threading even though solution 2 is more efficient
    let all_fish = read_fish();
    let chunk_size = (all_fish.len() as f32 / THREADS_MAX as f32).ceil() as usize;
    let fish_sectors: Vec<Vec<u32>> = all_fish.chunks(chunk_size).map(|e| e.to_vec()).collect();
    println!("{:?}", fish_sectors);

    let mut threads = Vec::new();
    for mut sector in fish_sectors {
        threads.push(thread::spawn(move || {
            for _day in 0..DAYS {
                simulate(&mut sector);
            }
            sector.len()
        }));
    }

    let total = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .sum::<usize>();

    println!("{total}");
    Some(total as u64)
}

fn read_fish() -> Vec<u32> {
    read_lines("6")
        .next()
        .unwrap()
        .split(",")
        .map(|e| e.parse::<u32>().unwrap())
        .collect()
}

fn simulate(fish: &mut Vec<u32>) {
    let mut new_fish = 0u32;
    for f in fish.iter_mut() {
        if *f == 0 {
            *f = 6;
            new_fish += 1;
        } else {
            *f -= 1;
        }
    }
    for _ in 0..new_fish {
        fish.push(8);
    }
}

pub fn part2() -> Option<u64> {
    const SIM_DAYS: u32 = 256;
    let fish_input = read_fish();
    let mut fish_colony = HashMap::new();
    for fish in fish_input {
        *fish_colony.entry(fish).or_insert(0u64) += 1;
    }
    for _day in 0..SIM_DAYS {
        let mut new_colony = HashMap::new();
        for (&days, &count) in fish_colony.iter() {
            if days == 0 {
                *new_colony.entry(6).or_insert(0) += count;
                *new_colony.entry(8).or_insert(0) += count;
            } else {
                *new_colony.entry(days - 1).or_insert(0) += count;
            }
        }
        fish_colony = new_colony;
    }
    let total: u64 = fish_colony.iter().map(|e| e.1).sum();
    println!("{:?}", fish_colony);
    println!("{total}");
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(385391, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1728611055389, part2().unwrap());
    }
}

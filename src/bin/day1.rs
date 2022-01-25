use std::env;

use aoc::read_lines;

pub fn part1() -> u32 {
    let mut larger_count = 0u32;
    let mut last_read: Option<u32> = None;

    for line in read_lines("1") {
        let depth = line.parse::<u32>().unwrap();
        if last_read.is_some() && last_read.unwrap() < depth {
            larger_count += 1;
        }
        last_read = Some(depth);
    }
    println!("Increases: {larger_count}");
    larger_count
}

pub fn part2() -> u32 {
    let mut larger_count = 0u32;
    const WINDOW_SIZE: usize = 3;
    let mut window: [u32; WINDOW_SIZE] = [0; 3];
    let mut window_sum = 0u32;
    let mut last_idx: usize = 0;

    let mut lines_it = read_lines("1");

    for i in 0..WINDOW_SIZE {
        if let Some(line) = lines_it.next() {
            let depth = line.parse::<u32>().unwrap();
            window[i] = depth;
            window_sum += depth;
        }
    }

    for line in lines_it {
        let depth = line.parse::<u32>().unwrap();
        let new_window_sum = window_sum + depth - window[last_idx];
        if new_window_sum > window_sum {
            larger_count += 1;
        }
        window_sum = new_window_sum;
        window[last_idx] = depth;
        last_idx = (last_idx + 1) % WINDOW_SIZE;
    }
    println!("Increases: {larger_count}");
    larger_count
}

fn main() {
    let part = env::args().nth(1).unwrap();
    match part.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => {
            println!("Invalid option");
            0
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1581, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1618, part2());
    }
}

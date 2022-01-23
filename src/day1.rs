use std::io::{self, BufRead};

pub fn part1() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut larger_count = 0u32;
    let mut last_read: Option<u32> = None;

    while let Some(line) = lines.next() {
        let depth = line.unwrap().parse::<u32>().unwrap();
        if last_read.is_some() && last_read.unwrap() < depth {
            larger_count += 1;
        }
        last_read = Some(depth);
    }
    println!("Increases: {larger_count}");
}

pub fn part2() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut larger_count = 0u32;
    const WINDOW_SIZE: usize = 3;
    let mut window: [u32; WINDOW_SIZE] = [0; 3];
    let mut window_sum = 0u32;
    let mut last_idx: usize = 0;

    for i in 0..WINDOW_SIZE {
        if let Some(line) = lines.next() {
            let depth = line.unwrap().parse::<u32>().unwrap();
            window[i] = depth;
            window_sum += depth;
        }
    }

    while let Some(line) = lines.next() {
        let depth = line.unwrap().parse::<u32>().unwrap();
        let new_window_sum = window_sum + depth - window[last_idx];
        if new_window_sum > window_sum {
            larger_count += 1;
        }
        window_sum = new_window_sum;
        window[last_idx] = depth;
        last_idx = (last_idx + 1) % WINDOW_SIZE;
    }
    println!("Increases: {larger_count}");
}
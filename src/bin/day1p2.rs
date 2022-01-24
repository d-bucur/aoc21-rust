mod common;
use crate::common::read_lines;

pub fn main() {
    let mut larger_count = 0u32;
    const WINDOW_SIZE: usize = 3;
    let mut window: [u32; WINDOW_SIZE] = [0; 3];
    let mut window_sum = 0u32;
    let mut last_idx: usize = 0;

    let mut lines_it = read_lines();

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
}
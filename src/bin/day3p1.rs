use aoc::read_lines;

pub fn main() {
    let length = 12;
    let mut total = 0u32;
    let mut ones = vec![0u32; length];

    for line in read_lines() {
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
}

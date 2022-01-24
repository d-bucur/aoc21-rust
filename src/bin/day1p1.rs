use aoc::read_lines;

pub fn main() {
    let mut larger_count = 0u32;
    let mut last_read: Option<u32> = None;

    for line in read_lines() {
        let depth = line.parse::<u32>().unwrap();
        if last_read.is_some() && last_read.unwrap() < depth {
            larger_count += 1;
        }
        last_read = Some(depth);
    }
    println!("Increases: {larger_count}");
}

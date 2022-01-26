use aoc::{read_lines, Vec2d};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn part1() -> u32 {
    let mut heights = Vec2d::new(WIDTH, HEIGHT, 0u32);
    for (y, line) in read_lines("9").enumerate() {
        for (x, h) in line.chars().map(|f| f.to_digit(10).unwrap()).enumerate() {
            heights.set(x, y, h);
        }
    }
    let mut total = 0;
    for y in 0..HEIGHT as i32 {
        for x in 0..WIDTH as i32 {
            let current_height = heights.at(x as usize, y as usize);
            let up = if let Some(adjacent_height) = heights.safe_at(x, y - 1) {
                current_height >= adjacent_height
            } else {
                false
            };
            let down = if let Some(adjacent_height) = heights.safe_at(x, y + 1) {
                current_height >= adjacent_height
            } else {
                false
            };
            let left = if let Some(adjacent_height) = heights.safe_at(x - 1, y) {
                current_height >= adjacent_height
            } else {
                false
            };
            let right = if let Some(adjacent_height) = heights.safe_at(x + 1, y) {
                current_height >= adjacent_height
            } else {
                false
            };
            if !(up || down || left || right) {
                // println!("local low: {x},{y} = {current_height}");
                total += current_height + 1;
            }
        }
    }
    // println!("{:?}", heights);
    println!("{total}");
    total
}

fn part2() -> u32 {
    // Implement union find https://www.cs.princeton.edu/~rs/AlgsDS07/01UnionFind.pdf
    0
}

fn main() {
    let part = std::env::args().nth(1).unwrap();
    match part.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => {
            println!("Invalid option");
            0u32
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(504, part1());
    }

    #[test]
    fn test_part2() {
        // assert_eq!(1010460, part2());
    }
}

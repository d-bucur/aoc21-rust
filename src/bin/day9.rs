use aoc::{read_lines, Vec2d};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn parse_input() -> Vec2d<u8> {
    let mut heights = Vec2d::new(WIDTH, HEIGHT, 0u8);
    for (y, line) in read_lines("9").enumerate() {
        for (x, h) in line
            .chars()
            .map(|f| f.to_digit(10).unwrap() as u8)
            .enumerate()
        {
            heights.set(x, y, h);
        }
    }
    heights
}

fn part1() -> u32 {
    let heights = parse_input();
    let mut total: u32 = 0;
    for y in 0..HEIGHT as i32 {
        for x in 0..WIDTH as i32 {
            let current_height = heights.at(x as usize, y as usize);
            let mut local_min = true;
            for (ni, nj) in vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)] {
                if let Some(adjacent_height) = heights.safe_at(ni, nj) {
                    if current_height >= adjacent_height {
                        local_min = false;
                        break;
                    }
                }
            }
            if local_min {
                // println!("local low: {x},{y} = {current_height}");
                total += current_height as u32 + 1;
            }
        }
    }
    // println!("{:?}", heights);
    println!("{total}");
    total
}

fn part2() -> u32 {
    // using DFS to count connected areas
    let mut basin_sizes = Vec::new();
    let heights = parse_input();
    let mut visited = Vec2d::new(WIDTH, HEIGHT, false);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if let Some(size) = visit(x, y, &mut visited, &heights) {
                basin_sizes.push(size);
            }
        }
    }
    basin_sizes.sort_unstable();
    let result = basin_sizes
        .into_iter()
        .rev()
        .take(3)
        .reduce(|a, i| a * i)
        .unwrap();
    println!("{result:?}");
    result
}

fn visit(x: usize, y: usize, visited: &mut Vec2d<bool>, heights: &Vec2d<u8>) -> Option<u32> {
    // do a BFS from start position
    if visited.at(x, y) || heights.at(x, y) == 9 {
        return None;
    }
    let mut to_visit = Vec::new();
    to_visit.push((x as i32, y as i32));
    visited.set(x, y, true);
    let mut size = 0u32;

    while let Some((i, j)) = to_visit.pop() {
        size += 1;
        for (ni, nj) in vec![(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)] {
            if let Some(height) = heights.safe_at(ni, nj) {
                if height != 9 && !visited.get_and_set(ni as usize, nj as usize, true) {
                    to_visit.push((ni, nj));
                }
            }
        }
    }
    Some(size)
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
        assert_eq!(1558722, part2());
    }
}

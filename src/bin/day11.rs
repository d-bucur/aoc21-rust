use aoc::{read_lines, Vec2d, Point};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn parse_input() -> Vec2d<u8> {
    let mut heights = Vec2d::new(WIDTH, HEIGHT, 0u8);
    for (y, line) in read_lines("11").enumerate() {
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

fn simulate(steps: u32) -> Option<u64> {
    let mut grid = parse_input();
    let mut flashes: u64 = 0;
    for _cycle in 0..steps {
        for (x, y) in grid.pos_iter() {
            increment_energy(&mut grid, x, y, &mut flashes);
        }
        for energy in grid.nums.iter_mut() {
            if *energy >= 10 {
                *energy = 0;
            }
        }
        // println!("Step {cycle}");
        // println!("{grid:?}");
    }
    println!("{flashes}");
    Some(flashes)
}

fn increment_energy(grid: &mut Vec2d<u8>, x: usize, y: usize, flashes: &mut u64) {
    let new_energy = grid.at(x, y) + 1;
    grid.set(x, y, new_energy);
    if new_energy == 10 {
        *flashes += 1;
        for Point((x, y)) in grid.neighbors(x, y).chain(grid.diagonals(x, y)) {
            increment_energy(grid, x, y, flashes)
        }
    }
}

fn part1() -> Option<u64> {
    simulate(100)
}

fn part2() -> Option<u64> {
    None
}

fn main() {
    let part = std::env::args().nth(1).unwrap();
    match part.as_str() {
        "1" => part1(),
        "2" => part2(),
        _ => {
            panic!("Invalid option");
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Some(1585), part1());
    }

    #[test]
    fn test_part2() {
        // assert_eq!(1190420163, part2());
    }
}

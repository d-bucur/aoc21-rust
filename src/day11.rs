use crate::read_lines;
use crate::Point;
use crate::Vec2d;

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

fn simulate(steps: u32, check_sync_flash: bool) -> Option<u64> {
    let mut grid = parse_input();
    let mut total_flashes: u64 = 0;
    let total_cells = (WIDTH * HEIGHT) as u64;

    for cycle in 0..steps {
        let mut step_flashes = 0;
        for (x, y) in grid.pos_iter() {
            increment_energy(&mut grid, x, y, &mut step_flashes);
        }
        for energy in grid.nums.iter_mut() {
            if *energy >= 10 {
                *energy = 0;
            }
        }
        // println!("Step {cycle}: {step_flashes}");
        if check_sync_flash && step_flashes == total_cells {
            println!("{}", cycle + 1);
            return Some(cycle as u64 + 1);
        }
        total_flashes += step_flashes;
        // println!("Step {cycle}");
        // println!("{grid:?}");
    }
    println!("{total_flashes}");
    Some(total_flashes)
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

pub fn part1() -> Option<u64> {
    simulate(100, false)
}

pub fn part2() -> Option<u64> {
    simulate(u32::max_value(), true)
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
        assert_eq!(382, part2().unwrap());
    }
}

use aoc::read_lines;
use aoc::Vec2d;
use regex::Regex;
use std::env;

const BOARD_SIZE: usize = 999;

fn solve(draw_diagonals: bool) -> usize {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let mut board = Vec2d::new(BOARD_SIZE, BOARD_SIZE, 0usize);
    for line in read_lines("5") {
        let caps = re.captures(&line).unwrap();
        let (x1, y1, x2, y2) = (
            caps[1].parse::<usize>().unwrap(),
            caps[2].parse::<usize>().unwrap(),
            caps[3].parse::<usize>().unwrap(),
            caps[4].parse::<usize>().unwrap(),
        );
        draw(&mut board, x1, y1, x2, y2);
        if draw_diagonals {
            draw_diagonal(&mut board, x1, y1, x2, y2);
        }
    }
    let answer = board.nums.iter().filter(|&&e| e >= 2).count();
    println!("{answer}");
    answer
}

fn my_range(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
    if a < b {
        Box::new(a..=b) as Box<dyn Iterator<Item = _>>
    } else {
        Box::new((b..=a).rev())
    }
}

fn draw_diagonal(board: &mut Vec2d<usize>, x1: usize, y1: usize, x2: usize, y2: usize) {
    if (x2 as i32 - x1 as i32).abs() != (y2 as i32 - y1 as i32).abs() {
        return;
    }
    let y_it = my_range(y1, y2);
    let x_it = my_range(x1, x2);
    let points = x_it.zip(y_it);
    for (x, y) in points {
        board.set(x, y, board.at(x, y) + 1);
    }
}

fn draw(board: &mut Vec2d<usize>, x1: usize, y1: usize, x2: usize, y2: usize) {
    if x1 == x2 {
        for y in my_range(y1, y2) {
            board.set(x1, y, board.at(x1, y) + 1);
        }
    } else if y1 == y2 {
        for x in my_range(x1, x2) {
            board.set(x, y1, board.at(x, y1) + 1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "1" => solve(false),
        "2" => solve(true),
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
        assert_eq!(5294, solve(false));
    }

    #[test]
    fn test_part2() {
        assert_eq!(21698, solve(true));
    }
}

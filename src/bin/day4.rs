use std::{collections::HashMap, env};

use aoc::{read_lines, Vec2d};

#[derive(Debug)]
pub struct BingoBoard {
    nums: Vec2d<u32>,
    marked_x: Vec<usize>,
    marked_y: Vec<usize>,
    num_pos: HashMap<u32, (usize, usize)>,
    pub score: u32,
}

impl BingoBoard {
    fn new(nums: Vec2d<u32>) -> BingoBoard {
        let marked_x = vec![0; nums.get_width()];
        let marked_y = vec![0; nums.get_height()];
        let mut bb = BingoBoard {
            nums,
            marked_x,
            marked_y,
            num_pos: HashMap::new(),
            score: 0,
        };
        let mut x = 0usize;
        let mut y = 0usize;
        let width = bb.nums.get_width();
        for num in bb.nums.nums.iter() {
            if x >= width {
                x = 0;
                y += 1;
            }
            let key = (x, y);
            bb.num_pos.insert(*num, key);
            x += 1;
            bb.score += num;
        }
        bb
    }

    pub fn mark(&mut self, num: u32) -> bool {
        if let Some((x, y)) = self.num_pos.get(&num) {
            self.marked_x[*x] += 1;
            self.marked_y[*y] += 1;
            self.score -= num;
            return self.marked_x[*x] == self.nums.get_width()
                || self.marked_y[*y] == self.nums.get_height();
        }
        return false;
    }
}

pub fn read_numbers(it: &mut impl Iterator<Item = String>) -> Vec<u32> {
    let numbers: Vec<u32> = it
        .next()
        .unwrap()
        .split(",")
        .map(|e| e.parse::<u32>().unwrap())
        .collect();
    it.next(); // skip first empty line
    numbers
}

const BOARD_SIZE: usize = 5;

pub fn read_boards(input_it: impl Iterator<Item = String>) -> HashMap<usize, BingoBoard> {
    let mut board = Vec2d::new(BOARD_SIZE, BOARD_SIZE, 0u32);
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut boards = HashMap::new();
    let mut id = 0usize;
    for line in input_it {
        if line == "" {
            boards.insert(id, BingoBoard::new(board));
            id += 1;
            board = Vec2d::new(BOARD_SIZE, BOARD_SIZE, 0u32);
            x = 0;
            y = 0;
        } else {
            for num in line.split_whitespace() {
                let n = num.parse::<u32>().unwrap();
                board.set(x, y, n);
                x += 1;
            }
            x = 0;
            y += 1;
        }
    }
    boards.insert(id, BingoBoard::new(board)); // save last board
    boards
}

pub fn part1() -> u32 {
    let mut it = read_lines("4");
    let numbers = read_numbers(&mut it);
    let mut boards = read_boards(it);

    for num in numbers {
        for (_, board) in boards.iter_mut() {
            if board.mark(num) {
                println!("Bingo! {}", num * board.score);
                return num * board.score;
            }
        }
    }
    return 0;
}

pub fn part2() -> u32 {
    let mut it = read_lines("4");
    let numbers = read_numbers(&mut it);
    let mut boards = read_boards(it);

    for num in numbers {
        let mut score = 0;
        boards.retain(|_, board| {
            let won = board.mark(num);
            if won {
                score = board.score;
            }
            !won
        });
        if boards.len() == 0 {
            println!(
                "Last Bingo! on {num}. Unmaked: {score}. Final score: {}",
                score * num
            );
            return score * num;
        }
    }
    return 0;
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
        assert_eq!(67716, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1830, part2());
    }
}

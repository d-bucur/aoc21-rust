mod common;
mod day4common;
use crate::common::read_lines;
use crate::day4common::*;

pub fn main() {
    let mut it = read_lines();
    let numbers = read_numbers(&mut it);
    let mut boards = read_boards(it);

    for num in numbers {
        for (_, board) in boards.iter_mut() {
            if board.mark(num) {
                println!("Bingo! {}", num * board.score);
                return;
            }
        }
    }
}
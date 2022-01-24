use aoc::read_lines;
mod day4common;
use day4common::*;

pub fn main() {
    let mut it = read_lines();
    let numbers = read_numbers(&mut it);
    let mut boards = read_boards(it);

    for num in numbers {
        let mut removal = Vec::new();
        let board_it = boards.iter_mut();
        for (id, board) in board_it {
            if board.mark(num) {
                removal.push(*id);
            }
        }
        if boards.len() == 1 && removal.len() > 0 {
            let score = boards.iter().next().unwrap().1.score;
            println!(
                "Last Bingo! on {num}. Unmaked: {score}. Final score: {}",
                score * num
            );
            return;
        }
        for i in removal.iter() {
            // println!("Bingo! on {num}. Removing board {i}. Remaining boards {}", boards.len());
            boards.remove(i);
        }
    }
}

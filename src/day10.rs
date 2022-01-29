use crate::read_lines;

pub fn part1() -> Option<u64> {
    let r = process_lines(true).0;
    println!("{r}");
    Some(r)
}

pub fn part2() -> Option<u64> {
    let r = process_lines(false).1;
    println!("{r}");
    Some(r)
}

enum Bracket {
    Round,
    Square,
    Curly,
    Angular,
}

fn process_lines(skip_completion_score: bool) -> (u64, u64) {
    let mut invalid_score = 0;
    let mut completion_scores = Vec::new();
    for line in read_lines("10") {
        let mut skip_line = false;
        let mut open: Vec<Bracket> = Default::default();
        for c in line.chars() {
            match c {
                '(' => open.push(Bracket::Round),
                '[' => open.push(Bracket::Square),
                '<' => open.push(Bracket::Angular),
                '{' => open.push(Bracket::Curly),
                ')' => {
                    if let Some(Bracket::Round) = open.pop() {
                    } else {
                        invalid_score += 3;
                        skip_line = true;
                        break;
                    }
                }
                ']' => {
                    if let Some(Bracket::Square) = open.pop() {
                    } else {
                        invalid_score += 57;
                        skip_line = true;
                        break;
                    }
                }
                '>' => {
                    if let Some(Bracket::Angular) = open.pop() {
                    } else {
                        invalid_score += 25137;
                        skip_line = true;
                        break;
                    }
                }
                '}' => {
                    if let Some(Bracket::Curly) = open.pop() {
                    } else {
                        invalid_score += 1197;
                        skip_line = true;
                        break;
                    }
                }
                _ => panic!("Invalid char"),
            }
        }
        if skip_line {
            continue;
        }
        if skip_completion_score {
            continue;
        }
        let mut incomplete_score: u64 = 0;
        while open.len() > 0 {
            let new_score = match open.pop().unwrap() {
                Bracket::Round => 1,
                Bracket::Square => 2,
                Bracket::Curly => 3,
                Bracket::Angular => 4,
            };
            incomplete_score = incomplete_score * 5 + new_score;
        }
        completion_scores.push(incomplete_score);
    }
    let mut completion_score = 0;
    if !skip_completion_score {
        completion_scores.sort_unstable();
        completion_score = completion_scores[completion_scores.len() / 2];
    }
    (invalid_score, completion_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(389589, part1().unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1190420163, part2().unwrap());
    }
}

use aoc::read_lines;

fn part2() -> u32 {
    0
}

enum Bracket {
    Round,
    Square,
    Curly,
    Angular,
}

fn part1() -> u32 {
    let mut total = 0;
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
                        total += 3;
                        skip_line = true;
                        break;
                    }
                }
                ']' => {
                    if let Some(Bracket::Square) = open.pop() {
                    } else {
                        total += 57;
                        skip_line = true;
                        break;
                    }
                }
                '>' => {
                    if let Some(Bracket::Angular) = open.pop() {
                    } else {
                        total += 25137;
                        skip_line = true;
                        break;
                    }
                }
                '}' => {
                    if let Some(Bracket::Curly) = open.pop() {
                    } else {
                        total += 1197;
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
    }
    println!("{total}");
    total
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
        assert_eq!(389589, part1());
    }

    #[test]
    fn test_part2() {
        // assert_eq!(1010460, part2());
    }
}

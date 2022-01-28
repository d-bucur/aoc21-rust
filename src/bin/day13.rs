use aoc::{read_lines, Vec2d};
use regex::Regex;
use termion::color;

const SET: u8 = 7;
const UNSET: u8 = 0;

fn part1() -> Option<usize> {
    let (mut dots, folds) = parse_input()?;
    for (axis, along) in folds.into_iter().take(1) {
        let width = dots.get_width();
        let height = dots.get_height();
        fold(&mut dots, &axis, along, width, height);
    }
    let result = dots
        .pos_iter()
        .filter(|(x, y)| dots.at(*x, *y) == SET)
        .count();
    println!("{result}");
    Some(result)
}

fn part2() -> Option<usize> {
    let (mut dots, folds) = parse_input()?;
    let mut width = dots.get_width();
    let mut height = dots.get_height();
    for (axis, along) in folds.into_iter() {
        fold(&mut dots, &axis, along, width, height);
        if axis == "x" {
            width /= 2;
        } else {
            height /= 2;
        }
    }
    for y in 0..height {
        for x in 0..width {
            let dot = dots.at(x, y);
            if dot == SET {
                print!("{}{},", color::Fg(color::Cyan), dot);
            } else {
                print!("{}{},", color::Fg(color::White), dot);
            }
        }
        println!();
    }
    None
}

fn fold(dots: &mut Vec2d<u8>, axis: &String, along: usize, width: usize, height: usize) {
    if axis == "y" {
        for y in along..height {
            for x in 0..width {
                let old = dots.get_and_set(x, y, UNSET);
                if old == SET {
                    dots.set(x, along - (y - along), SET)
                }
            }
        }
    }
    if axis == "x" {
        for y in 0..height {
            for x in along..width {
                let old = dots.get_and_set(x, y, UNSET);
                if old == SET {
                    dots.set(along - (x - along), y, SET)
                }
            }
        }
    }
}

fn parse_input() -> Option<(Vec2d<u8>, Vec<(String, usize)>)> {
    let re: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
    let re2: Regex = Regex::new(r"fold along ([xy])=(\d+)").unwrap();
    let mut input_it = read_lines("13");
    let points: Vec<_> = input_it
        .by_ref()
        .take_while(|l| l != "")
        .map(|line| {
            let caps = re.captures(&line).unwrap();
            let x = caps[1].parse::<usize>().unwrap();
            let y = caps[2].parse::<usize>().unwrap();
            (x, y)
        })
        .collect();
    let (width, _) = points.iter().max_by_key(|(x, _)| x)?;
    let (_, height) = points.iter().max_by_key(|(_, y)| y)?;
    // println!("{width} x {height}");
    let mut dots = Vec2d::new(*width + 1, *height + 1, UNSET);
    for (x, y) in points {
        dots.set(x, y, SET);
    }
    let folds: Vec<_> = input_it
        .map(|line| {
            let caps = re2.captures(&line).unwrap();
            let axis = caps[1].to_string();
            let along = caps[2].parse::<usize>().unwrap();
            (axis, along)
        })
        .collect();
    Some((dots, folds))
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
        assert_eq!(Some(753), part1());
    }

    #[test]
    fn test_part2() {
        // assert_eq!(HZLEHJRK, part2().unwrap());
    }
}

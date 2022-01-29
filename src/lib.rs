use std::{
    fmt::Debug,
    io::{BufRead, BufReader, Read},
};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn read_lines(day: &'static str) -> impl Iterator<Item = String> {
    let fin = std::fs::File::open(format!("inputs/day{}.input", day)).unwrap();
    lines_iter(fin)
}

fn lines_iter(fin: impl Read) -> impl Iterator<Item = String> {
    let buffer = BufReader::new(fin);
    let it = buffer.lines().map(|e| e.unwrap());
    it
}

pub struct Point<T>(pub (T, T));

pub struct Vec2d<T> {
    pub nums: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl<T> Vec2d<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, init: T) -> Vec2d<T> {
        Vec2d {
            nums: vec![init; width * height],
            width,
            height,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> T {
        self.nums[y * self.width + x].clone()
    }

    pub fn at_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        Some(&mut self.nums[y * self.width + x])
    }

    pub fn safe_at(&self, x: i32, y: i32) -> Option<T> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            Some(self.nums[y as usize * self.width + x as usize].clone())
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.nums[y * self.width + x] = val;
    }

    /// Get the current value in the position and set the new one to val
    pub fn get_and_set(&mut self, x: usize, y: usize, val: T) -> T {
        let idx = y * self.width + x;
        let previous = self.nums[idx].clone();
        self.nums[idx] = val;
        previous
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn pos_iter(&self) -> Vec2dPosIterator {
        Vec2dPosIterator {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec2dNeighborsIterator {
        let up = if y > 0 { Some(Point((x, y - 1))) } else { None };
        let down = if y < self.width - 1 {
            Some(Point((x, y + 1)))
        } else {
            None
        };
        let left = if x > 0 { Some(Point((x - 1, y))) } else { None };
        let right = if x < self.height - 1 {
            Some(Point((x + 1, y)))
        } else {
            None
        };
        Vec2dNeighborsIterator {
            directions: [up, left, right, down],
            i: 0,
        }
    }

    pub fn diagonals(&self, x: usize, y: usize) -> Vec2dNeighborsIterator {
        let d1 = if y > 0 && x > 0 {
            Some(Point((x - 1, y - 1)))
        } else {
            None
        };
        let d2 = if x < self.width - 1 && y < self.width - 1 {
            Some(Point((x + 1, y + 1)))
        } else {
            None
        };
        let d3 = if x > 0 && y < self.width - 1 {
            Some(Point((x - 1, y + 1)))
        } else {
            None
        };
        let d4 = if x < self.height - 1 && y > 0 {
            Some(Point((x + 1, y - 1)))
        } else {
            None
        };
        Vec2dNeighborsIterator {
            directions: [d1, d4, d3, d2],
            i: 0,
        }
    }
}

impl<T> Debug for Vec2d<T>
where
    T: Debug,
    T: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut x = 0usize;
        let width = self.get_width();
        for n in self.nums.iter() {
            f.write_fmt(format_args!("{:?}, ", n))?;
            x += 1;
            if x == width {
                f.write_str("\n")?;
                x = 0;
            }
        }
        Result::Ok(())
    }
}

pub struct Vec2dPosIterator {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Iterator for Vec2dPosIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.height {
            return None;
        }
        let (prev_x, prev_y) = (self.x, self.y);
        self.x += 1;
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        Some((prev_x, prev_y))
    }
}

pub struct Vec2dNeighborsIterator {
    directions: [Option<Point<usize>>; 4],
    i: usize,
}

impl Iterator for Vec2dNeighborsIterator {
    type Item = Point<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < 4 {
            let dir = self.directions[self.i].take();
            self.i += 1;
            if dir.is_some() {
                return dir;
            }
        }
        None
    }
}

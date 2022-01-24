use std::io::{self, BufRead, BufReader};

pub fn read_lines() -> impl Iterator<Item = String> {
    let stdin = io::stdin();
    let buffer = BufReader::new(stdin);
    let it = buffer.lines().map(|e| e.unwrap());
    it
}

#[derive(Debug)]
pub struct Vec2d<T> {
    pub nums: Vec<T>,
    pub width: usize,
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
        }
    }

    pub fn at(&self, x: usize, y: usize) -> T {
        self.nums[y * self.width + x].clone()
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.nums[y * self.width + x] = val;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.nums.len() / self.width
    }
}

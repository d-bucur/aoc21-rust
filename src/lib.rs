use std::{
    fmt::Debug,
    io::{self, BufRead, BufReader},
};

pub fn read_lines() -> impl Iterator<Item = String> {
    let stdin = io::stdin();
    let buffer = BufReader::new(stdin);
    let it = buffer.lines().map(|e| e.unwrap());
    it
}

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

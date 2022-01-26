use std::{
    fmt::Debug,
    io::{BufRead, BufReader, Read},
};

pub fn read_lines(day: &'static str) -> impl Iterator<Item = String> {
    let fin = std::fs::File::open(format!("inputs/day{}.input", day)).unwrap();
    lines_iter(fin)
}

pub fn lines_iter(fin: impl Read) -> impl Iterator<Item = String> {
    let buffer = BufReader::new(fin);
    let it = buffer.lines().map(|e| e.unwrap());
    it
}

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
            height
        }
    }

    pub fn at(&self, x: usize, y: usize) -> T {
        self.nums[y * self.width + x].clone()
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

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
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

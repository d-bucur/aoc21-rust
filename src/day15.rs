use std::{collections::{HashMap, BinaryHeap}, thread::current};

use crate::{read_lines, Vec2d, Point};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

#[derive(PartialEq, Eq, Debug)]
struct PqueueEntry {
    priority: i32,
    val: Point<usize>,
}

impl Ord for PqueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for PqueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// TODO replace with dijkstra and compare result
fn a_star(grid: &Vec2d<u32>, start: Point<usize>, end: Point<usize>) -> (HashMap<Point<usize>, Option<Point<usize>>>, HashMap<Point<usize>, u32>) {
    let mut frontier = BinaryHeap::new();
    frontier.push(PqueueEntry {priority: 0, val: start});
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    came_from.insert(start, None);
    cost_so_far.insert(start, 0u32);

    while let Some(curr) = frontier.pop() {
        // println!("Frontier: {:?}", frontier);
        println!("Visiting {},{} = {} with priority {}", curr.val.x, curr.val.y, grid.at(curr.val.x, curr.val.y), curr.priority);
        if curr.val == end {
            break;
        }

        for next in grid.neighbors(curr.val.x, curr.val.y) {
            let new_cost = cost_so_far[&curr.val] + grid.at(curr.val.x, curr.val.y);
            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                // println!("Better path found to {:?}, total cost {}", next, new_cost);
                cost_so_far.insert(next, new_cost);
                let priority = -((new_cost + heuristic(next, end)) as i32);
                frontier.push(PqueueEntry{priority, val: next});
                came_from.insert(next, Some(curr.val));
            }
        }
    }
    return (came_from, cost_so_far)
}

fn heuristic(next: Point<usize>, end: Point<usize>) -> u32 {
    ((next.x as i32 - end.x as i32).abs()
    + (next.y as i32 - end.y as i32).abs()) as u32
}

pub fn part1() -> Option<u64> {
    let mut grid = Vec2d::new(WIDTH, HEIGHT, 0);
    for (y, line) in read_lines("15").enumerate() {
        for (x, v) in line.chars().enumerate() {
            grid.set(x, y, v.to_digit(10).unwrap())
        }
    }
    let start = Point::new(0usize, 0usize);
    let end = Point::new(grid.get_width()-1, grid.get_height()-1);
    // println!("{grid:?}");
    let (came_from, cost_so_far) = a_star(&grid, start, end);
    println!("{:?}", cost_so_far);
    print_path(end, &came_from, &cost_so_far, &grid);
    println!("{:?}", cost_so_far.get(&end));
    None
}

fn print_path(end: Point<usize>, came_from: &HashMap<Point<usize>, Option<Point<usize>>>, cost_so_far: &HashMap<Point<usize>, u32>, grid: &Vec2d<u32>) {
    let mut curr = end;
    while curr != Point::new(0, 0) {
        println!("{:?}, value = {} total {:?}", curr, grid.at(curr.x, curr.y), cost_so_far.get(&curr));
        curr = came_from.get(&curr).unwrap().unwrap();
    }
}

pub fn part2() -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // assert_eq!(Some(2375), part1());
    }

    #[test]
    fn test_part2() {
        // assert_eq!(Some(1976896901756), part2());
    }
}

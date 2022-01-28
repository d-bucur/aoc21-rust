use std::collections::HashMap;

use aoc::read_lines;
use core::hash::Hash;
use regex::Regex;

#[derive(Debug)]
struct Graph<T> {
    edges: HashMap<usize, Vec<usize>>,
    node_to_id: HashMap<T, usize>,
    id_to_node: HashMap<usize, T>,
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    fn new() -> Self {
        Graph {
            edges: Default::default(),
            node_to_id: Default::default(),
            id_to_node: Default::default(),
        }
    }

    fn add_edge_undirected(&mut self, start: T, end: T) {
        self.add_edge_directed(start.clone(), end.clone());
        self.add_edge_directed(end, start);
    }

    fn add_edge_directed(&mut self, start: T, end: T) {
        let start_id = self.insert_mapping(start);
        let end_id = self.insert_mapping(end);
        self.edges
            .entry(start_id)
            .or_insert(Vec::new())
            .push(end_id);
    }

    fn insert_mapping(&mut self, edge: T) -> usize {
        let e = self.node_to_id.get(&edge);
        if let Some(id) = e {
            return *id;
        }
        let new_id = self.node_to_id.len();
        self.id_to_node.insert(new_id, edge.clone());
        self.node_to_id.insert(edge, new_id);
        new_id
    }
}

#[derive(Clone, PartialEq, Debug)]
enum NodeType {
    Small,
    Big,
    Terminal,
    Start,
}

#[derive(Debug)]
struct Visitor {
    graph: Graph<String>,
    check_twice: bool,
    node_types: Vec<NodeType>,
}

impl Visitor {
    fn new(graph: Graph<String>, check_twice: bool) -> Self {
        let mut node_types = vec![NodeType::Small; graph.id_to_node.len()];
        for (i, node) in graph.id_to_node.iter() {
            node_types[*i] = if node.to_uppercase() == *node {
                NodeType::Big
            } else if node.len() <= 2 {
                NodeType::Small
            } else if node == "end" {
                NodeType::Terminal
            } else {
                NodeType::Start
            }
        }
        Visitor {
            graph,
            check_twice,
            node_types,
        }
    }

    fn start_visit(&self) -> Vec<Vec<usize>> {
        let start_node = *self.graph.node_to_id.get(&"start".to_string()).unwrap();
        let mut visited: HashMap<usize, u32> = HashMap::new();
        let mut path: Vec<usize> = Vec::new();
        let mut valid_paths: Vec<Vec<usize>> = Vec::new();
        self.visit(start_node, &mut visited, &mut path, &mut valid_paths, false);
        valid_paths
    }

    fn visit(
        &self,
        current_node: usize,
        visited: &mut HashMap<usize, u32>,
        path: &mut Vec<usize>,
        valid_paths: &mut Vec<Vec<usize>>,
        visited_twice: bool,
    ) {
        // println!("-------");
        // println!("Current: {current_node}");
        // println!("Path: {path:?}");
        // println!("Visited: {visited:?}");
        path.push(current_node.clone());
        if self.node_types[current_node] == NodeType::Terminal {
            // println!("Valid path added to result: {path:?}");
            valid_paths.push(path.clone());
            path.pop();
            return;
        }
        let neighbors = self.graph.edges.get(&current_node).unwrap();
        let visited_count = visited.entry(current_node.clone()).or_insert(0u32);
        *visited_count += 1;
        neighbors.into_iter().for_each(|neigh| {
            let neigh_visits = *visited.get(neigh).unwrap_or(&0u32);
            let (should_visit, replace_visited_twice) = match self.node_types[*neigh] {
                NodeType::Big => (true, false),
                NodeType::Small => {
                    if !self.check_twice {
                        (neigh_visits == 0, false)
                    } else {
                        if neigh_visits == 0 {
                            (true, false)
                        } else {
                            // trying to visit small case twice
                            if visited_twice {
                                (false, true)
                            } else {
                                (true, true)
                            }
                        }
                    }
                }
                NodeType::Terminal => (neigh_visits == 0, false),
                NodeType::Start => (false, false),
            };
            if should_visit {
                self.visit(
                    neigh.clone(),
                    visited,
                    path,
                    valid_paths,
                    visited_twice || replace_visited_twice,
                );
            }
        });
        // println!("Exiting visit of {current_node}");
        *visited.get_mut(&current_node).unwrap() -= 1;
        path.pop();
        // println!("Popped {} from visited", p.unwrap());
    }
}

fn solve(check_twice: bool) -> Option<usize> {
    let re: Regex = Regex::new(r"(\w+)-(\w+)").unwrap();
    let mut nodes = Graph::new();
    for line in read_lines("12") {
        let caps = re.captures(&line)?;
        let start = caps[1].to_string();
        let end = caps[2].to_string();
        nodes.add_edge_undirected(start, end);
    }
    // println!("{nodes:?}");
    let visitor = Visitor::new(nodes, check_twice);
    // println!("{visitor:?}");
    let paths = visitor.start_visit();
    // println!("Valid paths to end: {paths:?}");
    println!("{}", paths.len());
    Some(paths.len())
}

fn part1() -> Option<usize> {
    solve(false)
}

fn part2() -> Option<usize> {
    solve(true)
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
        assert_eq!(Some(3000), part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(74222, part2().unwrap());
    }
}

use std::collections::HashMap;

use aoc::read_lines;
use core::hash::Hash;
use regex::Regex;

#[derive(Debug)]
struct Graph<T> {
    edges: Vec<Vec<usize>>,
    node_to_id: HashMap<T, usize>,
    id_to_node: HashMap<usize, T>,
}

#[allow(dead_code)]
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

        let size_max = std::cmp::max(start_id, end_id);
        if size_max >= self.edges.len() {
            self.edges.resize(size_max + 1, Default::default());
        }
        self.edges[start_id].push(end_id);
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

    fn ids_to_nodes(&self, ids: &Vec<usize>) -> Vec<T> {
        ids.into_iter()
            .map(|i| self.id_to_node.get(i).unwrap().clone())
            .collect()
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

    fn start_visit(&self) -> u64 {
        let start_node = *self.graph.node_to_id.get(&"start".to_string()).unwrap();
        let mut visited: Vec<usize> = vec![0; self.graph.node_to_id.len()];
        self.visit(start_node, &mut visited, false)
    }

    fn visit(&self, current_node: usize, visited: &mut Vec<usize>, visited_twice: bool) -> u64 {
        // println!("-------");
        // println!("Current: {}", self.graph.id_to_node[&current_node]);
        // println!("Path: {:?}", self.graph.ids_to_nodes(path));
        // println!("Visited: {:?}", visited);
        if self.node_types[current_node] == NodeType::Terminal {
            // println!("Valid path added to result: {:?}", self.graph.ids_to_nodes(path));
            return 1;
        }
        let neighbors = &self.graph.edges[current_node];
        visited[current_node] += 1;
        let valid_paths: u64 = neighbors
            .into_iter()
            .map(|neigh| {
                let neigh_visits = visited[*neigh];
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
                    self.visit(*neigh, visited, visited_twice || replace_visited_twice)
                } else {
                    0
                }
            })
            .sum();
        // println!("Exiting visit of {current_node}");
        visited[current_node] -= 1;
        valid_paths
        // println!("Popped {} from visited", p.unwrap());
    }
}

fn solve(check_twice: bool) -> Option<u64> {
    let re: Regex = Regex::new(r"(\w+)-(\w+)").unwrap();
    let mut nodes = Graph::new();
    for line in read_lines("12") {
        let caps = re.captures(&line)?;
        let start = caps[1].to_string();
        let end = caps[2].to_string();
        nodes.add_edge_undirected(start, end);
    }
    let visitor = Visitor::new(nodes, check_twice);
    // println!("{visitor:?}");
    let paths = visitor.start_visit();
    println!("{paths}");
    Some(paths)
}

fn part1() -> Option<u64> {
    solve(false)
}

fn part2() -> Option<u64> {
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

use std::collections::HashMap;
use crate::traits::solver::Solver;

pub struct Day08;

impl Day08 {
    fn lcm(&self, a: u128, b: u128) -> u128 {
        a * b / self.gcd(a, b)
    }

    fn gcd(&self, first: u128, second: u128) -> u128 {
        let mut max = first;
        let mut min = second;
        if min > max {
            let val = max;
            max = min;
            min = val;
        }

        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }

            max = min;
            min = res;
        }
    }
}

impl Solver for Day08 {
    fn day(&self) -> usize {
        8
    }

    fn name(&self) -> String {
        String::from("Haunted Wasteland")
    }

    fn solve_first(&self) -> u128 {
        let input = self.input_first();
        let lines = &mut input
            .lines()
            .filter(|line| !line.is_empty());
        let sequence = lines
            .next()
            .expect("No sequence found")
            .trim()
            .to_string();
        let node_descriptions = lines
            .map(|line| line.trim().to_string())
            .map(|line| {
                let mut parts = line.split(" = ");
                let name = parts
                    .nth(0)
                    .expect("No name found")
                    .to_string();
                let links = parts
                    .nth(0)
                    .expect("No links found")
                    .replace("(", "")
                    .replace(")", "")
                    .split(", ")
                    .map(|link| link.to_string())
                    .collect::<Vec<String>>();

                (name, links)
            })
            .collect::<Vec<(String, Vec<String>)>>();

        let mut nodes = HashMap::new();
        for (name, links) in node_descriptions.iter() {
            let left = links.get(0).expect("No left link found");
            let right = links.get(1).expect("No right link found");

            nodes.insert(
                name.clone(),
                Node::new(
                    name.clone(),
                    Some(left.clone()),
                    Some(right.clone()),
                )
            );
        }

        let mut current_node_name = nodes
            .values()
            .filter(|node| node.is_start())
            .last()
            .expect("No start node found")
            .name
            .clone();
        let mut step = 0;
        let mut current_node = nodes
            .get(&current_node_name)
            .expect("No start node found")
            .clone();
        while !current_node.is_end() {
            for char in sequence.chars() {
                step += 1;
                if current_node.is_end() {
                    break;
                }
                match char {
                    'L' => {
                        current_node_name = current_node.left.clone().expect("No left node name found");
                    }
                    'R' => {
                        current_node_name = current_node.right.clone().expect("No right node name found");
                    }
                    _ => {
                        panic!("Unknown direction {}", char);
                    }
                }
                current_node = nodes
                    .get(&current_node_name)
                    .expect("No start node found")
                    .clone();
            }
        }

        step as u128
    }

    fn solve_second(&self) -> u128 {
        let input = self.input_second();
        let lines = &mut input
            .lines()
            .filter(|line| !line.is_empty());
        let sequence = lines
            .next()
            .expect("No sequence found")
            .trim()
            .to_string();
        let node_descriptions = lines
            .map(|line| line.trim().to_string())
            .map(|line| {
                let mut parts = line.split(" = ");
                let name = parts
                    .nth(0)
                    .expect("No name found")
                    .to_string();
                let links = parts
                    .nth(0)
                    .expect("No links found")
                    .replace("(", "")
                    .replace(")", "")
                    .split(", ")
                    .map(|link| link.to_string())
                    .collect::<Vec<String>>();

                (name, links)
            })
            .collect::<Vec<(String, Vec<String>)>>();

        let mut nodes = HashMap::new();
        for (name, links) in node_descriptions.iter() {
            let left = links.get(0).expect("No left link found");
            let right = links.get(1).expect("No right link found");

            nodes.insert(
                name.clone(),
                GhostNode::new(
                    name.clone(),
                    left.clone(),
                    right.clone(),
                    name.ends_with("A"),
                    name.ends_with("Z"),
                )
            );
        }

        nodes
            .values()
            .filter(|node| node.is_start)
            .map(|node| node.name.clone())
            .map(|node_name| {
                let mut current_node_name = node_name.clone();
                let mut step = 0;
                let mut current_node = nodes
                    .get(&current_node_name)
                    .expect("No start node found")
                    .clone();
                while !current_node.is_end {
                    for char in sequence.chars() {
                        if current_node.is_end {
                            break;
                        }
                        step += 1;
                        match char {
                            'L' => {
                                current_node_name = current_node.left.clone();
                            }
                            'R' => {
                                current_node_name = current_node.right.clone();
                            }
                            _ => {
                                panic!("Unknown direction {}", char);
                            }
                        }
                        current_node = nodes
                            .get(&current_node_name)
                            .expect("No start node found")
                            .clone();
                    }
                }
                step
            })
            .fold(1, |acc, x| self.lcm(acc, x))
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day08a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day08b.txt").to_string()
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: Option<String>,
    right: Option<String>,
}

impl Node {
    fn new(name: String, left: Option<String>, right: Option<String>) -> Self {
        Self {
            name,
            left,
            right,
        }
    }

    fn is_start(&self) -> bool {
        self.name == "AAA"
    }

    fn is_end(&self) -> bool {
        self.name == "ZZZ"
    }
}

#[derive(Debug, Clone)]
struct GhostNode {
    name: String,
    left: String,
    right: String,
    is_start: bool,
    is_end: bool,
}

impl GhostNode {
    fn new(name: String, left: String, right: String, is_start: bool, is_end: bool) -> Self {
        Self {
            name,
            left,
            right,
            is_start,
            is_end
        }
    }
}
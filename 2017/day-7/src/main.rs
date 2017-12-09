extern crate regex;
extern crate petgraph;
extern crate itertools;

use std::collections::HashMap;

use regex::Regex;
use itertools::Itertools;

use petgraph::Graph;
use petgraph::algo::toposort;

fn main() {
    let text = include_str!("../input.txt");
    solve(text);
}

fn solve(input: &str) {
    let mut graph = Graph::<_, ()>::new();
    let re = Regex::new(r"([a-z]+ )\(([0-9]+)\)(?: -> )?(.+)?").unwrap();
    let mut nodes = HashMap::new();

    for capture in re.captures_iter(input) {
        let index = graph.add_node(Program::new(&capture[1].trim(), capture[2].parse::<u32>().unwrap()));
        nodes.insert(capture[1].trim().to_string(), index);
    }

    for capture in re.captures_iter(input) {
        for connected in capture[3].trim_right_matches("\r").split(", ") {
            if connected != "" {
                let base = nodes.get(&capture[1].trim().to_string()).unwrap();
                let conn = nodes.get(connected.trim()).unwrap();
                graph.update_edge(*base, *conn, ());
            }
        }
    }

    let sorted_graph = toposort(&graph, None).unwrap();
    println!("Root node: {}", graph[sorted_graph[0]].name);

    for &node in sorted_graph.iter().rev() {
        if !graph.neighbors(node).map(|x| graph[x].total).all_equal() {
            let (min, max) = graph.neighbors(node).map(|x| graph[x].total).minmax().into_option().unwrap();
            let (left, right): (Vec<_>, Vec<_>) = graph.neighbors(node).partition(|&x| graph[x].total == min);

            let unbalanced = if left.len() == 1 {
                &graph[left[0]]
            }
            else {
                &graph[right[0]]
            };

            println!("Unbalanced node: {:?}", unbalanced);
            println!("Adjusted weight: {}", unbalanced.weight + min - max);

            break;
        }

        graph[node].total += graph.neighbors(node).map(|x| graph[x].total).sum::<u32>();
    }
}

#[derive(Debug)]
struct Program {
    name: String,
    weight: u32,
    total: u32
}

impl Program {
    pub fn new(name: &str, weight: u32) -> Program {
        Program {
            name: name.to_string(),
            weight: weight,
            total: weight
        }
    }
}

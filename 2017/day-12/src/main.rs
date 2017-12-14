extern crate regex;
extern crate petgraph;

use petgraph::Graph;
use petgraph::visit::Dfs;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt").trim();
    solve(input);
}

fn solve(input: &str) {
    let mut graph = Graph::<_, ()>::new();
    let mut nodes = HashMap::new();
    let mut visited: HashSet<petgraph::prelude::NodeIndex> = HashSet::new();


    let re = Regex::new(r"(\d+) <-> ([, 0-9]+)").unwrap();

    for capture in re.captures_iter(input) {
        let index = graph.add_node(capture[1].to_string());
        nodes.insert(capture[1].to_string(), index);
    }

    for capture in re.captures_iter(input) {
        let base = nodes.get(&capture[1].to_string()).unwrap().clone();

        for e in capture[2].split(", ") {
            if !nodes.contains_key(e) {
                let index = graph.add_node(e.to_string());
                nodes.insert(e.to_string(), index);
            }

            let target = nodes.get(e).unwrap();
            graph.update_edge(base, *target, ());
        }
    }

    let mut dfs = Dfs::new(&graph, *nodes.get("0").unwrap());

    let mut counter = 0;
    while let Some(nx) = dfs.next(&graph) {
        counter += 1;
    }

    let mut components = 0;
    for node in nodes.iter() {
        if !visited.contains(node.1) {
            components += 1;
            let mut visitor = Dfs::new(&graph, *node.1);
            while let Some(n) = visitor.next(&graph) {
                visited.insert(n);
            }
        }
    }

    println!("Count: {}", counter);
    println!("Nodes: {}", graph.node_count());
    println!("Components: {}", components);
}


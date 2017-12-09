extern crate regex;
extern crate petgraph;

use std::collections::HashMap;

use regex::Regex;

use petgraph::Graph;
use petgraph::visit::Dfs;
use petgraph::graph::NodeIndex;

fn main() {
    let text = include_str!("../input.txt");
    part_2(text);

    // let split: Vec<Vec<_>> = text.lines().map(|x| x.split("->").collect()).collect();

    // let left: Vec<_> = split.iter().filter_map(|x| x.get(0)).map(|x| x.trim()).collect();
    // let right: Vec<_> = split.iter().filter_map(|x| x.get(1)).map(|x| x.trim()).collect();

    // let all_prgs: Vec<&str> = left.iter().flat_map(|x| x.split_whitespace())
    //                                      .filter(|x| !x.starts_with("(")).collect();

    // let stacked_prgs: Vec<&str> = right.iter().flat_map(|x| x.split(", ")).collect();

    // let base: Vec<_> = all_prgs.iter().filter(|x| !stacked_prgs.contains(x)).collect();

    // println!("{:?}", base);
}

fn part_2(input: &str) {
    let mut first = true;
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

    let &root = nodes.get("vgzejbd").unwrap();

    let mut dfs = Dfs::new(&graph, root);
    while let Some(n) = dfs.next(&graph) {
        println!("{:?}", graph.node_weight(n));
    }
}

#[derive(Debug)]
struct Program {
    name: String,
    weight: u32
}

impl Program {
    pub fn new(name: &str, weight: u32) -> Program {
        Program {
            name: name.to_string(),
            weight: weight
        }
    }
}

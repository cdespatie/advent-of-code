extern crate petgraph;

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::Dfs;

use std::collections::HashSet;

fn main() {
    let input = "jxqlasbh";
    solve(input);
}

fn solve(input: &str) {
    let base = input.to_string();
    let mut counter = 0;
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut graph = Graph::<(usize, usize), usize>::new();
    let mut graph_nodes: Vec<Vec<Option<NodeIndex>>> = Vec::new();
    let mut visited: HashSet<petgraph::prelude::NodeIndex> = HashSet::new();

    for j in 0..128 {
        let string: String = format!("{}-{}", base, j.to_string());
        let hash = knot_hash(string, 64);
        graph_nodes.push(Vec::new());

        let bits = to_bits(hash);
        map.push(bits.chars().flat_map(|x| x.to_digit(10)).collect());

        // Part 1
        for c in bits.chars() {
            match c {
                '1' => counter += 1,
                _   => ()
            };
        }

        // Part 2
        for (i, c) in bits.chars().enumerate() {
            if c == '1' {
                let node = graph.add_node((i, j));
                graph_nodes[j].push(Some(node));
            }
            else {
                graph_nodes[j].push(None);
            }
        }
    }

    for (j, row) in map.iter().enumerate() {
        for (i, entry) in row.iter().enumerate() {
            if *entry == 1 {
                if i > 0 && map[j][i - 1] == 1 {
                    graph.add_edge(graph_nodes[j][i].unwrap(), graph_nodes[j][i - 1].unwrap(), 0);
                }
                if i < 127 && map[j][i + 1] == 1 {
                    graph.add_edge(graph_nodes[j][i].unwrap(), graph_nodes[j][i + 1].unwrap(), 0);
                }
                if j > 0 && map[j - 1][i] == 1 {
                    graph.add_edge(graph_nodes[j][i].unwrap(), graph_nodes[j - 1][i].unwrap(), 0);
                }
                if j < 127 && map[j + 1][i] == 1 {
                    graph.add_edge(graph_nodes[j][i].unwrap(), graph_nodes[j + 1][i].unwrap(), 0);
                }
            }
        }
    }

    let mut components = 0;
    for row in graph_nodes.iter() {
        for node in row.iter() {
            match *node {
                Some(n) => {
                    if !visited.contains(&n) {
                        components += 1;
                        let mut visitor = Dfs::new(&graph, n);
                        while let Some(x) = visitor.next(&graph) {
                            visited.insert(x);
                        }
                    }
                },
                _ => ()
            };
        }
    }

    println!("{}", counter);
    println!("{}", components);
}

fn to_bits(input: String) -> String {
    let mut output = String::new();
    for c in input.chars() {
        output += &format!("{:04b}", u32::from_str_radix(&c.to_string(), 16).unwrap());
    }
    output
}

// Day 10 code

fn knot_hash(input: String, reps: usize) -> String {
    let mut start: usize = 0;
    let mut skip: usize = 0;
    let mut vector: Vec<usize> = Vec::new();
    let mut dense = Vec::new();
    let mut const_chars: Vec<usize> = vec![17, 31, 73, 47, 23];

    let mut parsed = Vec::new();
    for i in input.chars() {
        parsed.push(i as usize);
    }
    parsed.append(&mut const_chars);

    let ascii_chars: String = parsed.iter().map(|x| x.to_string())
        .collect::<Vec<String>>().join(",");

    for i in 0..256 { vector.push(i); }

    for _ in 0..reps {
        for n in ascii_chars.split(",") {
            let split = get_split(&vector, start, n.parse::<usize>().unwrap());

            for (i, &x) in split.iter().rev().enumerate() {
                vector[(start + i) % 256] = x;
            }

            start = (start + split.len() + skip) % 256;
            skip += 1;
        }
    }

    for i in 0..16 {
        let mut temp = 0;
        for j in 0..16 {
            temp = temp ^ vector[(i * 16) + j];
        }

        dense.push(temp);
    }

    convert_hex(dense)
}

fn convert_hex(input: Vec<usize>) -> String {
    input.iter().map(|x| format!("{:02x}", x)).collect::<Vec<_>>().join("")
}

fn get_split(input: &Vec<usize>, start: usize, len: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for i in 0..len {
        out.push(input[(start + i) % input.len()].clone());
    }
    out
}

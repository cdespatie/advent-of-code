extern crate regex;

use regex::Regex;

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
    let mut programs = Vec::new();
    let re = Regex::new(r"([a-z]+ )\(([0-9]+)\)(?: -> )?(.+)?").unwrap();

    for capture in re.captures_iter(input) {
        programs.push(Program::new(&capture[1], capture[2].parse::<u32>().unwrap()));
    }

    for capture in re.captures_iter(input) {
        let above: Vec<_> = capture[3].split(", ").collect();
        if above.len() > 0 {
            let pos: usize = programs.iter().position(|x| x.name == capture[1]).unwrap();
            let current = programs.get(pos).unwrap();

            current.children = programs.iter().filter(|x| above.contains(&x.name.as_str())).collect();
        }
    }

    println!("{:?}", programs);
}

#[derive(Debug)]
struct Program {
    name: String,
    weight: u32,
    children: Vec<Program>
}

impl Program {
    pub fn new(name: &str, weight: u32) -> Program {
        Program {
            name: name.to_string(),
            weight: weight,
            children: Vec::new()
        }
    }
}

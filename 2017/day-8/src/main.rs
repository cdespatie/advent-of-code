extern crate regex;

use regex::Regex;
use std::collections::HashMap;

// Groups:
// 1 2   3       4 5  6
// d dec -220 if h == 0

fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
    let input = include_str!("../input.txt");
    let re = Regex::new(r"([a-z]+) ([a-z]+) (-*\d+) if ([a-z]+) ([><!=]+) (-*\d+)").unwrap();

    let mut counter = 0;
    for cap in re.captures_iter(input) {
        counter += 1;
        let test_amt = if map.contains_key(&cap[4].to_string()) {
            *map.get(&cap[4].to_string()).unwrap()
        }
        else {
            0
        };

        let test = match &cap[5] {
            ">" => test_amt > cap[6].parse::<i32>().unwrap(),
            "<" => test_amt < cap[6].parse::<i32>().unwrap(),
            "==" => test_amt == cap[6].parse::<i32>().unwrap(),
            "!=" => test_amt != cap[6].parse::<i32>().unwrap(),
            ">=" => test_amt >= cap[6].parse::<i32>().unwrap(),
            "<=" => test_amt <= cap[6].parse::<i32>().unwrap(),
            _ => { println!("shit"); false }
        };

        if test {
            let oper = cap[3].parse::<i32>().unwrap();
            match &cap[2] {
                "inc" => *map.entry(cap[1].to_string()).or_insert(oper) += oper,
                "dec" => *map.entry(cap[1].to_string()).or_insert(-oper) -= oper,
                _ => ()
            };
        }
    }

    println!("{:?}", counter);
    println!("{:?}", map);
    println!("{:?}", map.iter().max_by_key(|&(_, val)| val).unwrap());
}


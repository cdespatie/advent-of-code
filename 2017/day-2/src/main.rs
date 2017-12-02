use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();

    let mut s = String::new();
    file.read_to_string(&mut s).ok();

    let input : Vec<Vec<u32>> = s.trim().lines()
        .map(|x| x.split_whitespace().map(|y| y.parse::<u32>().unwrap()).collect()).collect();

    let ans_1 = checksum_1(&input);
    let ans_2 = checksum_2(&input);

    println!("part 1: {}", ans_1);
    println!("part 2: {}", ans_2);
}

fn checksum_1(input: &Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        sum += line.iter().max().unwrap() - line.iter().min().unwrap();
    }
    sum
}

fn checksum_2(input: &Vec<Vec<u32>>) -> u32 {
    let mut sum: u32 = 0;

    for line in input {
        for &x in line {
            for &y in line {
                if x != y && x % y == 0 {
                    sum += x / y;
                }
            }
        }
    }
    sum
}


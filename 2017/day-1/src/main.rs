use std::fs::File;
use std::io::prelude::*;
use std::path::path;

fn main() {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();

    let mut s = String::new();
    file.read_to_string(&mut s).ok();

    let ints : Vec<u32> = s.trim().chars()
        .map(|x|
             x.to_digit(10).unwrap()
        ).collect();

    part_1(&ints);
    part_2(&ints);
}

fn part_1(input: &Vec<u32>) {
    let mut sum: u32 = 0;

    for (i, &x) in input.iter().enumerate() {
        if x == input[(i + 1) % input.len()] {
            sum += x;
        }
    }

    println!("part 1: {}", sum);
}

fn part_2(input: &Vec<u32>) {
    let mut sum: u32 = 0;

    for (i, &x) in input.iter().enumerate() {
        if x == input[(i + input.len() / 2) % input.len()] {
            sum += x;
        }
    }

    println!("part 2: {}", sum);
}


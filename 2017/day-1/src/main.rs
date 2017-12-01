use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();

    let mut s = String::new();
    file.read_to_string(&mut s);

    let mut ints : Vec<u32> = s.trim().chars()
        .map(|x|
             x.to_digit(10).unwrap()
        ).collect();

    part_1(&mut ints);
    part_2(&mut ints);
}

fn part_2(input: &mut Vec<u32>) {
    let mut sum: u32 = 0;
    let length = input.len();

    for (i, x) in input.iter().enumerate() {
        let mut index = 0;

        if (length / 2) + i > length {
            index = length - (length / 2 + i);
        }
        else {
            index = (length / 2) + i;
        }
        
        if *x == input[index] {
            sum = &sum + x;
        }
    }

    println!("--- part 2 ---");
    println!("length: {}", length);
    println!("sum: {}", sum);
}

fn part_1(input: &mut Vec<u32>) {
    let mut sum: u32 = 0;
    let mut prev: u32 = 0;
    let last: u32 = *input.last().unwrap();

    for x in input.iter() {
        if x == &prev {
            sum = &sum + x;
        }
        
        prev = *x;
    }

    if last == input[0] {
        sum = sum + last;
    }
    
    println!("--- part 1 ---");
    println!("{:?}", sum);
}

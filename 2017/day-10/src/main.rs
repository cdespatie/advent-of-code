fn main() {
    let input = include_str!("../input.txt").trim();
    solve(input);
}

fn solve(input: &str) {
    let mut vector: Vec<usize> = Vec::new();
    let mut start: usize = 0;
    for i in 0..256 { vector.push(i); }

    for (skip, n) in input.split(",").enumerate() {
        let split = get_split(&vector, start, n.parse::<usize>().unwrap());

        for (i, &x) in split.iter().rev().enumerate() {
            vector[(start + i) % 256] = x;
        }

        start = (start + split.len() + skip) % 256;
    }

    println!("{:?}", vector);
    println!("Part 1: {}", vector[0] * vector[1]);
}

fn get_split(input: &Vec<usize>, start: usize, len: usize) -> Vec<usize> {
    let mut out = Vec::new();
    for i in 0..len {
        out.push(input[(start + i) % input.len()].clone());
    }
    out
}

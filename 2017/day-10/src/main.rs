fn main() {
    let input = include_str!("../input.txt").trim();
    let mut const_chars: Vec<usize> = vec![17, 31, 73, 47, 23];

    let mut parsed = Vec::new();
    for i in input.chars() {
        parsed.push(i as usize);
    }
    println!("{:?}", parsed);
    parsed.append(&mut const_chars);
    println!("{:?}", parsed);

    let ascii_chars: String = parsed.iter().map(|x| x.to_string())
        .collect::<Vec<String>>().join(",");

    knot_hash(ascii_chars, 64);
}

fn knot_hash(input: String, reps: usize) {
    let mut start: usize = 0;
    let mut skip: usize = 0;
    let mut vector: Vec<usize> = Vec::new();

    for i in 0..256 { vector.push(i); }

    for rep in 0..reps {
        for n in input.split(",") {
            let split = get_split(&vector, start, n.parse::<usize>().unwrap());

            for (i, &x) in split.iter().rev().enumerate() {
                vector[(start + i) % 256] = x;
            }

            start = (start + split.len() + skip) % 256;
            skip += 1;
        }
    }

    let mut dense = Vec::new();

    for i in 0..16 {
        let mut temp = 0;
        for j in 0..16 {
            temp = temp ^ vector[(i * 16) + j];
        }

        dense.push(temp);
    }

    println!("{:?}", dense);
    println!("{:?}", convert_hex(dense));
}

fn convert_hex(input: Vec<usize>) -> String {
    input.iter().map(|x| format!("{:x}", x)).collect::<Vec<_>>().join("")
}

fn part_1(input: &str) {
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

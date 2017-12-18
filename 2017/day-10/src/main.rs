fn main() {
    let input = include_str!("../input.txt").trim();
    let mut const_chars: Vec<usize> = vec![17, 31, 73, 47, 23];

    let mut parsed = Vec::new();
    for i in input.chars() {
        parsed.push(i as usize);
    }
    parsed.append(&mut const_chars);

    let ascii_chars: String = parsed.iter().map(|x| x.to_string())
        .collect::<Vec<String>>().join(",");

    println!("Part 1: {:?}", part_1(input));
    println!("Part 2: {:?}", knot_hash(ascii_chars, 64));
}

fn knot_hash(input: String, reps: usize) -> String {
    let mut start: usize = 0;
    let mut skip: usize = 0;
    let mut vector: Vec<usize> = Vec::new();
    let mut dense = Vec::new();

    for i in 0..256 { vector.push(i); }

    for _ in 0..reps {
        for n in input.split(",") {
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

fn part_1(input: &str) -> usize {
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

    vector[0] * vector[1]
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

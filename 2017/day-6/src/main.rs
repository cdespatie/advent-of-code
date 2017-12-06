use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let parsed_input: Vec<u32> = input.trim().split_whitespace()
        .map(|x| x.parse::<u32>().unwrap()).collect();

    println!("Input: {:?}", parsed_input);
    reallocate(&mut parsed_input.clone());
}

fn reallocate(input: &mut Vec<u32>) {
    let input_size = input.len();
    let mut hash: HashSet<Vec<_>> = HashSet::new();
    let mut toggle = false;
    let mut p1_counter = 0;
    let mut p2_counter = 0;

    loop {
        hash.insert(input.clone());
        let (i, &val) = input.iter().enumerate()
            .max_by_key(|&(i, val)| (val, -(i as isize))).unwrap();

        input[i] = 0;
        for j in 0..val as usize {
            input[(i + j + 1) % input_size] += 1;
        }

        match toggle {
            true => p2_counter += 1,
            _    => p1_counter += 1
        };

        if hash.contains(input) {
            if toggle {
                println!("p1 - #iters: {}\np2 - #iters: {}", p1_counter, p2_counter);
                break;
            }

            hash = HashSet::new();
            toggle = true;
        }
    }
}


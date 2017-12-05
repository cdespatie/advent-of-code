fn main() {
    let input = include_str!("../input.txt");
    let parsed: Vec<i32> = input.trim().lines().map(|x| x.parse::<i32>().unwrap()).collect();

    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}

fn part_1(input: &Vec<i32>) -> u32 {
    let mut vector = input.clone();

    let mut counter: u32 = 0;
    let mut curr_index: i32 = 0;
    let max_index: i32 = vector.len() as i32;

    while curr_index >= 0 && curr_index < max_index {
        let temp = curr_index as usize;
        curr_index = vector[curr_index as usize] + curr_index;
        vector[temp] += 1;

        counter += 1;
    }
    counter
}

fn part_2(input: &Vec<i32>) -> u32 {
    let mut vector = input.clone();

    let mut counter: u32 = 0;
    let mut curr_index: i32 = 0;
    let max_index: i32 = vector.len() as i32;

    while curr_index >= 0 && curr_index < max_index {
        let temp = curr_index as usize;
        curr_index = vector[curr_index as usize] + curr_index;
        vector[temp] = if vector[temp] >= 3 { vector[temp] - 1 } else { vector[temp] + 1 };

        counter += 1;
    }
    counter
}

fn main() {
    let input = include_str!("../input.txt");

    let parsed = input
        .trim()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    println!("{}", part_1(&parsed));
    println!("{}", part_2(&parsed));
}

fn part_1(input: &Vec<i32>) -> i32 {
    input.iter()
        .map(|module_weight| (module_weight / 3) - 2)
        .sum()
}

fn part_2(input: &Vec<i32>) -> i32 {
    let mut copy = input.clone();
    let mut finished: bool = false;
    let mut total: i32 = 0;

    while !finished {
        for i in copy.iter_mut() {
            *i = (*i / 3) - 2;
        }

        copy.retain(|&x| x > 0);

        if copy.len() == 0 {
            finished = true;
        }

        total = total + copy.iter().sum::<i32>();
    }

    total
}

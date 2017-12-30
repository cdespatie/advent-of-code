fn main() {
    let input = include_str!("../input.txt").trim();
    let parsed_input: Vec<&str> = input.split(',').collect();

    solve(parsed_input);
}

fn solve(input: Vec<&str>) {
    let mut programs: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];

    // Cycle was detected at 24th iteration.
    // 1B % 24 == 16, therefore we only have to do
    // 16 calculations to find the final value.
    for x in 0..16 {
        for i in input.iter() {
            let mut data: Vec<usize> = Vec::new();
            let mut char_data: Vec<char> = Vec::new();

            if i.starts_with('s') {
                data = i.replace("s", "").split("/").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

                let temp = programs.clone();
                let n = programs.len() - data[0];

                programs.truncate(n);
                programs.splice(0..0, temp[n..].iter().cloned());
            }
            else if i.starts_with('x') {
                data = i.replace("x", "").split("/").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                programs.swap(data[0], data[1]);
            }
            else if i.starts_with('p') {
                char_data = i[1..].split("/").map(|x| x.parse::<char>().unwrap()).collect::<Vec<char>>();

                let pos_1 = programs.iter().position(|&x| x == char_data[0]).unwrap();
                let pos_2 = programs.iter().position(|&x| x == char_data[1]).unwrap();

                programs.swap(pos_1, pos_2);
            }
        }

        if x == 0 {
            println!("Part 1: {}", programs.iter().collect::<String>());
        }
    }

    println!("Part 2: {}", programs.into_iter().collect::<String>());
}

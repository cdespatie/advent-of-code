fn main() {
    let input = include_str!("../input.txt").trim();
    // let input = "0: 3\n1: 2\n4: 4\n6: 4";
    solve(input);
}

fn solve(input: &str) {
    let mut map = Vec::new();

    for line in input.lines() {
        let items: Vec<&str> = line.split(": ").collect();
        map.push((items[0].parse::<usize>().unwrap(), items[1].parse::<usize>().unwrap()));
    }

    map.sort_by_key(|&(x, _)| x);

    let mut counter = 0;
    for i in 0..map.last().unwrap().0 + 1 {
        match map.iter().find(|x| x.0 == i) {
            Some(x) => {
                if i % ((x.1 - 1) * 2) == 0 {
                    counter += x.0 * x.1;
                }
            },
            _ => continue
        };
    }

    println!("{}", counter);
}

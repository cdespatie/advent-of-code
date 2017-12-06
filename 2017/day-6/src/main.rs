fn main() {
    let input = include_str!("../input.txt");
    let parsed_input: Vec<u32> = input.trim().split_whitespace()
        .map(|x| x.parse::<u32>().unwrap()).collect();

    println!("{:?}", parsed_input);
    println!("{}", convert(&parsed_input));
}

fn part_1(input: &Vec<u32>) {
}

fn convert(input: &Vec<u32>) -> String {
    input.iter().map(|x| x.to_string()).collect::<Vec<String>>().connect(",")
}

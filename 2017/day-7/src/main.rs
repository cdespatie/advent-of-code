fn main() {
    let text = include_str!("../input.txt");
    let split: Vec<Vec<_>> = text.lines().map(|x| x.split("->").collect()).collect();

    let left: Vec<_> = split.iter().filter_map(|x| x.get(0)).map(|x| x.trim()).collect();
    let right: Vec<_> = split.iter().filter_map(|x| x.get(1)).map(|x| x.trim()).collect();

    let all_prgs: Vec<&str> = left.iter().flat_map(|x| x.split_whitespace())
                                         .filter(|x| !x.starts_with("(")).collect();

    let stacked_prgs: Vec<&str> = right.iter().flat_map(|x| x.split(", ")).collect();

    let base: Vec<_> = all_prgs.iter().filter(|x| !stacked_prgs.contains(x)).collect();

    println!("{:?}", base);
}


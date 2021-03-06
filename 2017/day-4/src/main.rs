use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let parsed: Vec<Vec<&str>> = input.trim().lines()
        .map(|x| x.split_whitespace().collect()).collect();

    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for line in parsed {
        if check_duplicates(&line) {
            sum_1 += 1;
        }

        if check_anagrams(&line) {
            sum_2 += 1;
        }
    }

    println!("part 1: {}", sum_1);
    println!("part 2: {}", sum_2);
}

fn check_anagrams(input: &Vec<&str>) -> bool {
    let mut hash = HashSet::new();
    let sorted = input.iter().map(|x| sort_word(x));

    for word in sorted {
        if hash.contains(&word) {
            return false;
        }
        hash.insert(word);
    }

    true
}

fn check_duplicates(input: &Vec<&str>) -> bool {
    let mut hash = HashSet::new();

    for word in input {
        if hash.contains(word) {
            return false;
        }
        hash.insert(word);
    }

    true
}

fn sort_word(input: &str) -> String {
    let mut word: Vec<char> = input.chars().collect();

    word.sort_by(|a, b| b.cmp(a));
    word.into_iter().collect::<String>()
}

// After solving, I discovered this wasn't necessary!
// Anagrams can be found simply by sorting the words and checking
// for duplicates. D'oh. Keeping this around; should be useful in
// future problems. Maybe extract to crate?

fn get_permutations(input: &str) -> Vec<String> {
    let mut output = Vec::new();
    permutate(input.len(), &mut input.chars().collect(), &mut output);

    output
}

fn permutate(n: usize, input: &mut Vec<char>, result: &mut Vec<String>) {
    if n == 1 {
        result.push(input.iter().clone().collect::<String>());
    }
    else {
        for i in 0..(n-1) {
            permutate(n - 1, input, result);

            if n % 2 == 0 {
                input.swap(i, n - 1);
            }
            else {
                input.swap(0, n - 1);
            }
        }

        permutate(n - 1, input, result);
    }
}


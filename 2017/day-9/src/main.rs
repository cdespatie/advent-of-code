fn main() {
    let input = include_str!("../input.txt");
    solve(input);
}

fn solve(input: &str) {
    let mut prev = ' ';
    let mut is_garbage = false;
    let mut score: u32 = 0;
    let mut garbage_counter: u32 = 0;
    let mut stack: Vec<u32> = Vec::new();

    for c in input.chars() {
        if prev == '!' {
            prev = ' ';
            continue;
        }

        if c == '{' && !is_garbage {
            let length: u32 = stack.len() as u32;
            stack.push(length + 1);
        }

        if c == '}' && !is_garbage {
            score += stack.pop().unwrap();
        }

        if is_garbage && c != '!' && c != '>' {
            garbage_counter += 1;
        }

        if c == '<' && !is_garbage {
            is_garbage = true;
        }
        else if c == '>' && is_garbage {
            is_garbage = false;
        }

        prev = c;
    }

    println!("Score: {}, Garbage: {}", score, garbage_counter);
}

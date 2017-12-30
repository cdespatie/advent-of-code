fn main() {
    part_1(377);
    part_2(377);
}

fn part_2(input: usize) {
    let mut ans = 0;
    let mut position = 0;

    for i in 1..50000000 {
        let index = ((input + position) % i) + 1;

        if index == 1 {
            ans = i;
        }
        
        position = index;
    }

    println!("{}", ans);
}

fn part_1(input: usize) {
    let mut position = 0;
    let mut buffer: Vec<usize> = vec![0];

    for i in 1..2018 {
        let index = ((input + position) % buffer.len()) + 1;

        if index > buffer.len() {
            buffer.push(i);
        }
        else {
            buffer.insert(index, i);
        }
        
        position = index;
    }

    let fin_index = (buffer.iter().position(|&x| x == 2017).unwrap() + 1) % buffer.len();

    println!("{:?}", buffer[fin_index]);
    println!("{:?}", buffer[fin_index - 1]);
}

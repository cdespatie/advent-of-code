fn main() {
    let mut gen_a = 634;
    let mut gen_b = 301;
    let mut counter_p1 = 0;
    let mut counter_p2 = 0;

    // Part 1
    for _ in 0..40000000 {
        gen_a = generator(gen_a, 16807);
        gen_b = generator(gen_b, 48271);

        if (gen_a & 0xFFFF) == (gen_b & 0xFFFF) {
            counter_p1 += 1;
        }
    }

    // Part 2
    let mut vec_a: Vec<u32> = Vec::new();
    let mut vec_b: Vec<u32> = Vec::new();
    gen_a = 634;
    gen_b = 301;

    while vec_a.len() < 5000000 {
        gen_a = generator(gen_a, 16807);

        if gen_a % 4 == 0 {
            vec_a.push(gen_a);
        }
    }

    while vec_b.len() < 5000000 {
        gen_b = generator(gen_b, 48271);

        if gen_b % 8 == 0 {
            vec_b.push(gen_b);
        }
    }

    for (i, a) in vec_a.iter().enumerate() {
        let b = vec_b[i];

        if (a & 0xFFFF) == (b & 0xFFFF) {
            counter_p2 += 1;
        }
    }

    println!("{}", counter_p1);
    println!("{}", counter_p2);
}

fn generator(value: u32, factor: u32) -> u32 {
    ((value as u64 * factor as u64) % 2147483647) as u32
}

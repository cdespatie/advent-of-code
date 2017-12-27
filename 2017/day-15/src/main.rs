fn main() {
    let mut gen_a = 634;
    let mut gen_b = 301;
    let mut counter = 0;

    for _ in 0..40000000 {
        gen_a = generator(gen_a, 16807);
        gen_b = generator(gen_b, 48271);

        if (gen_a & 0xFFFF) == (gen_b & 0xFFFF) {
            counter += 1;
        }
    }

    println!("{}", counter);
}

fn generator(value: u32, factor: u32) -> u32 {
    ((value as u64 * factor as u64) % 2147483647) as u32
}

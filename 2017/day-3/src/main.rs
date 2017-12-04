use std::collections::HashMap;

fn main() {
    //312051
    let (x, y) = get_coords(312051);
    let dist = manhat_dist(0, 0, x, y);
    println!("target: {}, {}", x, y);
    println!("distance: {}", dist);
}

fn get_coords(input: u32) -> (i32, i32) {
    let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut dx, mut dy): (i32, i32) = (0, -1);

    for _ in 1..input {
        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }

        x += dx;
        y += dy;
    }

    (x, y)
}

fn get_val(input: i32) {
    let mut dict = HashMap::new();

    // Insert base point.
    dict.insert(Point::new(0, 0), 1);

    let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut dx, mut dy): (i32, i32) = (0, -1);

    for _ in 1..input {
        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }

        x += dx;
        y += dy;
    }
}

fn manhat_dist(x_0: i32, y_0: i32, x_1: i32, y_1: i32) -> i32 {
    (x_1 - x_0).abs() + (y_1 - y_0).abs()
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

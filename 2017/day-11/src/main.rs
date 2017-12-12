#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x: x, y: y, z: z }
    }

    pub fn add(&mut self, delta: Point) {
        self.x += delta.x;
        self.y += delta.y;
        self.z += delta.z;
    }

    pub fn distance(&self, target: &Point) -> i32 {
        ((self.x - target.x).abs() + (self.y - target.y).abs() + (self.z - target.z).abs()) / 2
    }
}

fn main() {
    let input = include_str!("../input.txt").trim();
    solve(input);
}

fn solve(input: &str) {
    let center = Point::new(0, 0, 0);
    let mut position = Point::new(0, 0, 0);
    let mut all_distances: Vec<i32> = Vec::new();

    for i in input.split(",") {
        position.add(dir_to_coord(i));
        all_distances.push(position.distance(&center));
    }

    println!("Part 1: {:?}", position.distance(&center));
    println!("Part 2: {:?}", all_distances.iter().max().unwrap());
}

fn dir_to_coord(input: &str) -> Point {
    match input {
        "n"  => Point::new(0, 1, -1),
        "ne" => Point::new(1, 0, -1),
        "se" => Point::new(1, -1, 0),
        "s"  => Point::new(0, -1, 1),
        "sw" => Point::new(-1, 0, 1),
        "nw" => Point::new(-1, 1, 0),
        _    => Point::new(0, 0, 0)
    }
}

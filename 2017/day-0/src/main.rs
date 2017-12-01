use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");

    let mut file = match File::open(&path) {
        Err(error) => panic!("Error: {}", error.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => panic!(),
        Ok(_) => print!("{}", s)
    };

    let sani = s.replace(" ", "");
    let vec : Vec<&str> = sani.split(",").collect();
    print!("{:?}", vec);
}

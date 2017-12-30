use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let parsed = input.lines().map(|x| x.split(" ").collect::<Vec<&str>>()).collect::<Vec<_>>();

    solve(parsed);
}

fn solve(input: Vec<Vec<&str>>) {
    let mut last_sound = 0;
    let mut program_counter: i64 = 0;
    let mut registers = Registers::new();

    while program_counter < input.len() as i64 && program_counter >= 0 {
        let curr = &input[program_counter as usize];

        match curr[0] {
            "snd" => {
                last_sound = registers.get_val(curr[1]);
                program_counter += 1;
            },
            "set" => {
                let b = registers.get_val(curr[2]);
                registers.save(curr[1], b);
                program_counter += 1;
            },
            "add" => {
                let a = registers.get_val(curr[1]);
                let b = registers.get_val(curr[2]);

                registers.save(curr[1], a + b);
                program_counter += 1;
            },
            "mul" => {
                let a = registers.get_val(curr[1]);
                let b = registers.get_val(curr[2]);

                registers.save(curr[1], a * b);
                program_counter += 1;
            },
            "mod" => {
                let a = registers.get_val(curr[1]);
                let b = registers.get_val(curr[2]);

                registers.save(curr[1], a % b);
                program_counter += 1;
            },
            "rcv" => {
                let a = registers.get_val(curr[1]);

                if a != 0 {
                    println!("Recovered: {}", last_sound);
                    println!("{:?}", registers);
                    break;
                }

                program_counter += 1;
            },
            "jgz" => {
                let a = registers.get_val(curr[1]);
                let b = registers.get_val(curr[2]);

                if a > 0 {
                    program_counter += b;
                }
                else {
                    program_counter += 1;
                }
            },
            _ => {
                println!("OH NO");
                break;
            }
        }
        println!("instr: {:?}", curr);
        println!("{:?}", registers);
    }
}

#[derive(Debug)]
struct Registers {
    hash: HashMap<String, i64>
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            hash: HashMap::new()
        }
    }

    pub fn save(&mut self, key: &str, val: i64) {
        let string = key.to_string();
        *self.hash.entry(string).or_insert(0) = val;
    }

    pub fn get_val(&mut self, input: &str) -> i64 {
        let string = input.to_string();

        match input.parse::<i64>() {
            Ok(x) => x,
            Err(_) => {
                if self.hash.contains_key(&string) {
                    self.hash[&string]
                }
                else {
                    self.hash.insert(string, 0);
                    0
                }
            }
        }
    }
}

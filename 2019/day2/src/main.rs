fn main() {
    let program = "2,3,0,3,99";
    run(program
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect());
}

fn run(program: Vec<u32>) {
    todo!();
}

trait OpCode {
    const OP_CODE: u32;

    fn new(args: Vec<u32>) -> Self;
    fn run(&self) -> u32;
    fn get_args(&self) -> Vec<u32>;
    fn set_args(self, args: Vec<u32>);
}

impl OpCode for Add {
    const OP_CODE: u32 = 1;

    fn new(args: Vec<u32>) -> Add {
        Add {
            args: args
        }
    }

    fn run(&self) -> u32 {
        self.args[0] + self.args[1]
    }

    fn get_args(&self) -> Vec<u32> {
        self.args.clone()
    }

    fn set_args(mut self, args: Vec<u32>) {
        self.args = args;
    }
}

impl OpCode for Multiply {
    const OP_CODE: u32 = 2;

    fn new(args: Vec<u32>) -> Multiply {
        Multiply {
            args: args
        }
    }

    fn run(&self) -> u32 {
        self.args[0] * self.args[1]
    }

    fn get_args(&self) -> Vec<u32> {
        self.args.clone()
    }

    fn set_args(mut self, args: Vec<u32>) {
        self.args = args;
    }
}

struct Add {
    args: Vec<u32>,
}

struct Multiply {
    args: Vec<u32>,
}


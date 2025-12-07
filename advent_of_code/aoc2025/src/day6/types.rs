
#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply,
}

impl Operation {
    pub fn from(c: char) -> Self {
        match c {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => panic!("Could not match {} with any operation", c),
        }
    }
    pub fn toChar(&self) -> char {
        match self {
            Operation::Add => '+',
            Operation::Multiply => '*',
        }
    }
}

#[derive(Debug)]
pub struct Column {
    pub numbers: Vec<u64>,
    pub operation: Operation,
}

// New types for problem 2
pub type VerticalNumber = Vec<u64>;

#[derive(Debug)]
pub struct Problem {
    pub numbers: Vec<VerticalNumber>,
    pub operation: Operation,
}

impl Problem {
    pub fn pretty_print(&self) {
        let char = self.operation.toChar();
        println!("({})", char);
        for num in &self.numbers {
            println!("{:?}", num);
        }
        println!("===");
    }
}
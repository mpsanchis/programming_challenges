use crate::util::Part;

mod types;
mod input;
mod calculator;

use input::parse_input;
use types::{Machine};
use calculator::{MachineCalculator, MachineCalculatorResult};

fn main_1(mut machines: Vec<Machine>) {
    machines.reverse();
    let mut results: Vec<MachineCalculatorResult> = Vec::new();
    
    while let Some(machine) = machines.pop() {
        let machine_calculator = MachineCalculator::new(machine);
        results.push(machine_calculator.calculate_result());
        //eprintln!("Finishing loop for one iteration for debugging purposes");
        //break;
    }
    
    for result in &results {
        result.print();
    }
    println!("Final result: {}", results.iter().map(|r| r.len()).sum::<usize>());
}

pub fn main(part: &Part) {
    let machines = parse_input();
    match part {
        Part::One => {
            main_1(machines);
        }
        Part::Two => {
            println!("Part 2: {}", machines.len());
        }
    }
}
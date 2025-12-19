use crate::util::{Part, logger};

mod types;
mod input;
mod calculator;

use input::parse_input;
use types::{Machine};
use calculator::{MachineCalculator, MachineCalculatorResult, MachineCalculator2, MachineCalculatorResult2};

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

fn main_2(mut machines: Vec<Machine>) {
    machines.reverse();
    let mut results: Vec<MachineCalculatorResult2> = Vec::new();
    let mut iter = 0;
    
    while let Some(machine) = machines.pop() {
        println!("Starting iteration {}", iter);
        let mut machine_calculator = MachineCalculator2::new(machine);
        results.push(machine_calculator.calculate_result());
        iter += 1;
    }
    
    for result in &results {
        result.print();
    }
    println!("Final result: {}", results.iter().map(|r| r.num_presses()).sum::<usize>());
}

pub fn main(part: &Part) {
    let machines = parse_input();
    match part {
        Part::One => {
            main_1(machines);
        }
        Part::Two => {
            main_2(machines);
        }
    }
}
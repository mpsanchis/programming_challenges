use crate::util::{Part};

mod types;
mod types_ilp;
mod input;
mod input_ilp;
mod calculator;

use input::parse_input;
use input_ilp::parse_input_ilp;
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

fn main_2() {
    let input = parse_input_ilp();
    let mut solutions = Vec::new();
    
    for problem in input {
        let solution = problem.solve();
        solutions.push(solution);
    }
    
    for solution in &solutions {
        println!("{:?}", solution);
        println!("TOTAL {:?}", solution.iter().sum::<f64>());
    }
    let final_solution: f64 = solutions.iter().map(|solution| solution.iter().sum::<f64>()).sum();
    println!("FINAL TOTAL {:?}", final_solution);
}

pub fn main(part: &Part) {
    let machines = parse_input();
    match part {
        Part::One => {
            main_1(machines);
        }
        Part::Two => {
            main_2();
        }
    }
}
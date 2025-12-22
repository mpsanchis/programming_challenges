use crate::util::{Part, logger};

mod types;
mod input;
mod solver;

use input::parse_input;
use types::Graph;
use solver::GraphSolver;

fn main_1(graph: Graph) {
    let mut graph_solver = GraphSolver::new(&graph);
    let ways_out = graph_solver.find_ways_out("you");
    println!("Number of ways out: {:?}", ways_out);
}

fn main_2() {
    
}

pub fn main(part: &Part) {
    let graph = parse_input();
    match part {
        Part::One => {
            main_1(graph);
        }
        Part::Two => {
            main_2();
        }
    }
}

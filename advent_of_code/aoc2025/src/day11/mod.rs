use crate::util::{Part};

mod types;
mod input;
mod solver;

use input::parse_input;
use types::Graph;
use solver::{GraphSolver};

fn main_1(graph: Graph) {
    let mut graph_solver = GraphSolver::new(&graph);
    let ways_out = graph_solver.find_ways_out("you", "out");
    println!("Number of ways out: {:?}", ways_out);
}

fn main_2(graph: Graph) {
    let mut graph_solver = GraphSolver::new(&graph);
    
    let svr_to_fft = graph_solver.find_ways_out("svr", "fft").unwrap_or(0);
    println!("Number of ways from svr to fft: {}", svr_to_fft);
    graph_solver.reset_cache();
    let fft_to_dac = graph_solver.find_ways_out("fft", "dac").unwrap_or(0);
    println!("Number of ways from fft to dac: {}", fft_to_dac);
    graph_solver.reset_cache();
    let dac_to_out = graph_solver.find_ways_out("dac", "out").unwrap_or(0);
    println!("Number of ways from dac to out: {}", dac_to_out);
    graph_solver.reset_cache();
    println!("---");
    println!("result: {}", svr_to_fft as u64 * fft_to_dac as u64 * dac_to_out as u64);
    println!("---");
}

pub fn main(part: &Part) {
    let graph = parse_input();
    match part {
        Part::One => {
            main_1(graph);
        }
        Part::Two => {
            main_2(graph);
        }
    }
}

/**
 * Usage as seen in:
 * https://github.com/rust-or/good_lp/blob/effe018a0c63d884bf5c5c220bd71e39ddb3e03c/tests/resource_allocation_problem.rs
 */
use good_lp::*;

pub struct MinimumPressesProblem {
    coefficients: Coefficients,
    joltages: Vec<u32>,
}

pub type Coefficients = Vec<Vec<bool>>;

impl MinimumPressesProblem {
    pub fn new(joltages: Vec<u32>, coefficients: Coefficients) -> Self {
        MinimumPressesProblem { 
            coefficients,
            joltages, 
        }
    }

    fn get_constraints(&self, x: &Vec<Variable>) -> Vec<Constraint> {
        let mut constraints = Vec::new();
        for c in 0..self.joltages.len() {
            let coefficients_row = &self.coefficients[c];
            let constraint_expr: Expression = x.iter().enumerate()
                .filter(|(i, _)| coefficients_row[*i])
                .map(|(_, var)| var)
                .sum();
            let constraint = constraint_expr.eq(self.joltages[c]);
            constraints.push(constraint);
        }
        constraints
    }
    
    pub fn solve(&self) -> Vec<f64> {
        let num_vars = self.coefficients.get(0).unwrap().len();
        
        let mut problem_vars = variables! {};
        let vars = vec![variable().integer().min(0); num_vars];
        let x: Vec<Variable> = problem_vars.add_all(vars);
        
        let constraints = self.get_constraints(&x);
        let objective: Expression = x.iter().sum();
        
        let solution = problem_vars
            .minimise(objective)
            .using(default_solver)
            .with_all(constraints)
            .solve()
            .unwrap();
        
        x.iter().map(|var| solution.value(*var)).collect()
    }
}
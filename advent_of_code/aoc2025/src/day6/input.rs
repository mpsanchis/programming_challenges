use std::fs;
use crate::util::{Part, logger};

use super::types::{Column, Operation, Problem, VerticalNumber};

fn initialise_columns(columns: &mut Vec<Vec<u64>>, line: &str) {
    let numbers: Vec<u64> = line.split(" ")
        .filter(|n| !n.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();
    for number in numbers {
        columns.push(vec![number]);
    }
}

pub fn parse_input_1() -> Vec<Column> {
    let input = fs::read_to_string(file!().replace(".rs", ".txt"))
        .expect("could not read input");

    // contains only numbers
    let mut number_columns: Vec<Vec<u64>> = Vec::new();
    // contains only operations
    let mut op_columns: Vec<Operation> = Vec::new();

    for (line_num, line) in input.lines().enumerate() {
        if line_num == 0 {
            initialise_columns(&mut number_columns, line);
        } else {
            let num_or_sym: Vec<String> = line.split(" ")
                .filter(|word| !word.is_empty())
                .map(|word| word.to_string())
                .collect();
            match num_or_sym[0].parse::<u64>() {
                Ok(_) => {
                    // still numbers
                    let nums: Vec<u64> = num_or_sym.iter().map(|n| n.parse().unwrap()).collect();
                    for (col, num) in nums.iter().enumerate() {
                        number_columns[col].push(*num);
                    }
                },
                Err(_) => {
                    // operations: we have reached the end
                    let symbols = num_or_sym;
                    for sym in symbols {
                        op_columns.push(match sym.as_str() {
                            "*" => Operation::Multiply,
                            "+" => Operation::Add,
                            _ => panic!("Could not match symbol {} with any operation", sym),
                        });
                    }
                    break;
                }
            }
        }
    }
    let mut columns: Vec<Column> = Vec::new();
    for _ in 0..number_columns.len() {
        columns.push(Column {
            numbers: number_columns.pop().unwrap(),
            operation: op_columns.pop().unwrap(),
        });
    }
    columns
}

fn get_empty_problems(input: &str) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    let last_line = input.lines().last().unwrap();

    let mut opt_numbers_per_current_problem: Option<usize> = None;

    for c in last_line.chars() {
        if c == '+' || c == '*' {
            match opt_numbers_per_current_problem {
                // First operation found
                None => {
                    opt_numbers_per_current_problem = Some(1);
                    problems.push(Problem {
                        numbers: vec![],
                        operation: Operation::from(c)
                    });
                },
                // Operation found in between: stop local count and add to vector
                Some(numbers_per_current_problem) => {
                    // Generate 'numbers_per_current_problem-1' numbers in the last problem
                    let last_problem = problems.len()-1;
                    for _ in 0..(numbers_per_current_problem - 1) {
                        problems[last_problem].numbers.push(Vec::new());
                    }
                    // Add a new empty problem with the current operation
                    problems.push(Problem {
                        numbers: vec![],
                        operation: Operation::from(c)
                    });
                    // Reset the local count
                    opt_numbers_per_current_problem = Some(1)
                },

            }
        } else { // empty space: keep counting up
            let numbers_per_current_problem = opt_numbers_per_current_problem.unwrap();
            opt_numbers_per_current_problem = Some(numbers_per_current_problem + 1)
        }
    }
    // Generate 'numbers_per_current_problem' numbers in the last problem
    let last_problem = problems.len()-1;
    // logger().logn(&format!("Last problem has {} numbers", opt_numbers_per_current_problem.unwrap()));
    for _ in 0..opt_numbers_per_current_problem.unwrap() {
        problems[last_problem].numbers.push(Vec::new());
    }

    problems
}

pub fn parse_input_2() -> Vec<Problem> {
    let input = fs::read_to_string(file!().replace(".rs", ".txt"))
        .expect("could not read input");

    let mut problems = get_empty_problems(&input);

    for line in input.lines() {
        let mut problem_index = 0;
        let mut number_index = 0;

        logger().logn(&format!("Reading line: {}", line));
        for char in line.chars() {
            if number_index == problems[problem_index].numbers.len() {
                logger().logn(&format!("Index = {}. Increasing problem number", number_index));
                number_index = 0;
                problem_index = (problem_index + 1) % problems.len();
                continue;
            }
            logger().logn(&format!("At problem {} number {}", problem_index, number_index));
            if char.is_digit(10) {
                logger().logn(&format!("Found digit {}", char));
                problems[problem_index].numbers[number_index].push(char.to_digit(10).expect(&format!("Could not parse char '{}' as a digit. Is whitespace? {}", char, char.is_whitespace())) as u64);
            } else {
                logger().logn(&format!("Skipping char as is not digit"));
            }
            number_index += 1;
        }
    }
    problems
}

#[cfg(test)]
mod tests {
    use crate::day6::types::{Operation, Problem};

    use super::get_empty_problems;

    fn compare_problems(expected: &[Problem], actual: &[Problem]) {
        for (i, problem) in actual.iter().enumerate() {
            println!("Problem {i}");
            assert_eq!(problem.numbers.len(), expected[i].numbers.len());
            assert_eq!(problem.operation, expected[i].operation);
        }
    }

    #[test]
    fn get_column_widths_tst1() {
        let input = "*   +  *   + ";
        // assert_eq!(get_empty_problems(input), vec![3,2,3,2]);
        let expected_problems: [ Problem ; 4] = [
            Problem { numbers: vec![vec![]; 3], operation: Operation::Multiply },
            Problem { numbers: vec![vec![]; 2], operation: Operation::Add },
            Problem { numbers: vec![vec![]; 3], operation: Operation::Multiply },
            Problem { numbers: vec![vec![]; 2], operation: Operation::Add },
        ];
        compare_problems(&expected_problems, &get_empty_problems(input));
    }
    #[test]
    fn get_column_widths_tst2() {
        let input = "*   + * + ";
        let expected_problems: [ Problem ; 4] = [
            Problem { numbers: vec![vec![]; 3], operation: Operation::Multiply },
            Problem { numbers: vec![vec![]; 1], operation: Operation::Add },
            Problem { numbers: vec![vec![]; 1], operation: Operation::Multiply },
            Problem { numbers: vec![vec![]; 2], operation: Operation::Add },
        ];
        compare_problems(&expected_problems, &get_empty_problems(input));
    }
    #[test]
    fn get_column_widths_tst3() {
        let input = "*     +";
        let expected_problems: [ Problem ; 2] = [
            Problem { numbers: vec![vec![]; 5], operation: Operation::Multiply },
            Problem { numbers: vec![vec![]; 1], operation: Operation::Add },
        ];
        compare_problems(&expected_problems, &get_empty_problems(input));
    }
    #[test]
    fn get_column_widths_tst4() {
        let input = "+ +";
        let expected_problems: [ Problem ; 2] = [
            Problem { numbers: vec![vec![]; 1], operation: Operation::Add },
            Problem { numbers: vec![vec![]; 1], operation: Operation::Add },
        ];
        compare_problems(&expected_problems, &get_empty_problems(input));
    }
}
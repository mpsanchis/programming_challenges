use std::{usize};
use std::collections::{VecDeque};

use super::types::SwitchesPressed;
use crate::util::logger;

use super::types::{Machine, State};

pub struct MachineCalculator {
    machine: Machine,
}

struct StateToTry {
    pub local_result: SwitchesPressed,
    pub state_to_try: State,
    pub switch_id_to_try: usize,
}

impl MachineCalculator {
    pub fn new(machine: Machine) -> Self {
        MachineCalculator { 
            machine, 
        }
    }
    
    pub fn calculate_result(self) -> MachineCalculatorResult {
        let desired_state = self.machine.get_desired_state();
        let switches = self.machine.get_switches();
        let mut result = SwitchesPressed::new();
        let mut best_result = usize::MAX;
        // Discard the easy case where desired state is all off
        if desired_state.is_all_off() {
            return MachineCalculatorResult { 
                desired_state: desired_state.clone(),
                result,
            }
        }
        // Prepare a queue of jobs to do
        let mut states_to_try: VecDeque<StateToTry> = VecDeque::new();
        states_to_try.push_back(StateToTry { 
            local_result: result.clone(),
            state_to_try: State::new(desired_state.len()), 
            switch_id_to_try: 1,
        });
        let mut local_result_pressed = result.clone();
        local_result_pressed.push(switches[0].clone());
        states_to_try.push_back(StateToTry { 
            local_result: local_result_pressed,
            state_to_try: State::new(desired_state.len()).apply_switch(&switches[0]), 
            switch_id_to_try: 1,
        });
        while !states_to_try.is_empty() {
            let StateToTry { 
                local_result,
                state_to_try,
                switch_id_to_try, 
            } = states_to_try.pop_front().unwrap();
            if state_to_try == *desired_state {
                // Cut tree if reached the goal
                logger().logn(&format!("Found desired state: {}", desired_state));
                logger().logn(&format!("\tWith result: {}", local_result));
                if local_result.len() < best_result {
                    best_result = local_result.len();
                    result = local_result;
                }
                continue;
            }
            if switch_id_to_try > switches.len() - 1 {
                // Don't try more switches if reached the end
                continue;
            }
            logger().logn(&format!("Current state (not desired): {}", state_to_try));
            logger().logn(&format!("\tfrom switches: {}", &switches[switch_id_to_try]));
            logger().logn(&format!("\tlocal result: {}", &local_result));
            logger().logn(&format!("Pushing new switches to explore..."));
            // Solution not found: add two jobs
            // Don't press the switch
            states_to_try.push_back(StateToTry {
               local_result: local_result.clone(), 
               state_to_try: state_to_try.clone(), 
               switch_id_to_try: switch_id_to_try + 1,
            });
            // Press the switch
            let mut new_local_result = local_result.clone(); 
            new_local_result.push(switches[switch_id_to_try].clone());
            states_to_try.push_back(StateToTry { 
                local_result: new_local_result, 
                state_to_try: state_to_try.apply_switch(&switches[switch_id_to_try]), 
                switch_id_to_try: switch_id_to_try + 1, 
            });
        }
        
        MachineCalculatorResult { 
            desired_state: desired_state.clone(),
            result,
        }
    }
}

// Different struct to represent a change of state as defined in the rust book
pub struct MachineCalculatorResult {
    desired_state: State,
    result: SwitchesPressed,
}

impl MachineCalculatorResult {
    pub fn print(&self) {
        println!("Desired State: {}", self.desired_state);
        println!("Result: {}", self.result);
    }
    
    pub fn len(&self) -> usize {
        self.result.len()
    }
}

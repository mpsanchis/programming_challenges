use std::vec;
use std::{collections::HashMap, usize};
use std::collections::VecDeque;

use crate::day10::types::SwitchesPressed;
use crate::util::logger;

use super::types::{Machine, State, Joltage, Switch};

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

pub struct MachineCalculator2 {
    machine: Machine,
    result: Option<State2>,
}

impl MachineCalculator2 {
    pub fn new(machine: Machine) -> Self {
        MachineCalculator2 { machine, result: None }
    }
    
    fn new_state(&self, state: &State2, switch_id_to_try: usize) -> State2 {
        let mut new_state = state.clone();
        let current_attempts_switch = state.get(&switch_id_to_try).unwrap_or(&0);
        new_state.insert(switch_id_to_try, *current_attempts_switch + 1);
        new_state
    }
    
    fn calculate_joltages(&self, state: &State2) -> Vec<Joltage> {
        logger().logn(&format!("[calculate_joltages] Current state: {:?}", state));
        let mut joltages: Vec<Joltage> = vec![0; self.machine.get_joltages().len()];
        for (switch_id, num_presses) in state.iter() {
            logger().logn(&format!("[calculate_joltages] Adding joltages for switch #{switch_id}: {}", self.machine.get_switches().get(*switch_id).unwrap() ));
            let switch = &self.machine.get_switches()[*switch_id];
            for joltage_id in switch.iter() {
                joltages[*joltage_id] += *num_presses as u32;
            }
        }
        logger().logn(&format!("[calculate_joltages] Joltages: {:?}", joltages));
        joltages
    }
    
    fn are_joltages_equal(&self, j1: &Vec<Joltage>, j2: &Vec<Joltage>) -> bool {
        for i in 0..j1.len() {
            if j1[i] != j2[i] {
                return false;
            }
        }
        true
    }
    
    fn are_joltages_too_high(&self, desired_joltages: &Vec<Joltage>, current_joltages: &Vec<Joltage>) -> bool {
        for i in 0..desired_joltages.len() {
            if current_joltages[i] > desired_joltages[i] {
                return true;
            }
        }
        false
    }
    
    fn calculate_result_recursively(&mut self, state: State2) -> bool {
        logger().logn(&format!("Current state: {:?}", state));
        let desired_joltages = self.machine.get_joltages();
        let current_joltages = self.calculate_joltages(&state);
        logger().logn(&format!("Current joltages: {:?}", current_joltages));
        
        if self.are_joltages_equal(desired_joltages, &current_joltages) {
            // reached the result we wanted
            self.result = Some(state.clone());
            return true;
        }
        if self.are_joltages_too_high(desired_joltages, &current_joltages) {
            // Stop searching if pressing buttons doesn't make sense anymore
            return false;
        }
        // Explore pressing all the buttons
        for switch_id in 0..self.machine.get_switches().len() {
            let new_state = self.new_state(&state, switch_id);
            logger().logn(&format!("Trying to press switch: {switch_id} = {}", self.machine.get_switches().get(switch_id).unwrap()));
            let result_found = self.calculate_result_recursively(new_state);
            if result_found {
                // No need to keep looping
                return true;
            }
        }
        
        false
    }
    
    pub fn calculate_result(&mut self) -> MachineCalculatorResult2 {
        let initial_state = HashMap::new();
        let result_found = self.calculate_result_recursively(initial_state);
        
        if !result_found {
            panic!("Could not find a result for part 2");
        }
        
        let result = self.result.as_ref().unwrap();
        
        MachineCalculatorResult2 { 
            desired_joltages: self.machine.get_joltages().clone(), 
            result: result.clone(),
            switches: self.machine.get_switches().clone(),
        }
    }
}

pub type State2 = HashMap<
    usize, // id of the Switch being pressed
    usize // number of times the Switch has been pressed 
>;

pub struct MachineCalculatorResult2 {
    desired_joltages: Vec<Joltage>,
    result: State2,
    switches: Vec<Switch>,
}

impl MachineCalculatorResult2 {
    pub fn print(&self) {
        println!("Desired Joltages: {:?}", self.desired_joltages);
        println!("Obtained with:");
        let mut total_presses = 0;
        for (switch_id, num_presses) in self.result.iter() {
            let switch = self.switches.get(*switch_id).unwrap();
            println!("\tswitch: {}", switch);
            println!("\tpressed times: {}", num_presses);
            total_presses += *num_presses;
        }
        println!("\ttotal pressed times: {}", total_presses);
        println!("+++++");
    }
    
    pub fn num_presses(&self) -> usize {
        self.result.iter()
            .map(|(_,num_presses)| num_presses)
            .sum()
    }
}
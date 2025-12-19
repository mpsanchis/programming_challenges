use std::{usize, vec};
use std::collections::{HashSet, VecDeque};

use super::types::SwitchesPressed;
use super::combinatorics::CompositionCalculator;
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
    state_cache: HashSet<State2>,
    composition_calculator: CompositionCalculator,
}

enum CurrentVSDesired {
    Larger,
    Equal,
    Smaller,
}

impl MachineCalculator2 {
    pub fn new(machine: Machine) -> Self {
        MachineCalculator2 { 
            machine, 
            result: None,
            state_cache: HashSet::new(),
            composition_calculator: CompositionCalculator::new(),
        }
    }
    
    fn new_state(state: &State2, switch_id_to_try: usize) -> State2 {
        let mut new_state = state.clone();
        let current_attempts_switch = state.get(&switch_id_to_try).unwrap();
        new_state.insert(switch_id_to_try, *current_attempts_switch + 1);
        new_state
    }
    
    fn calculate_joltages(&self, state: &State2) -> Vec<Joltage> {
        //logger().logn(&format!("[calculate_joltages] Current state: {:?}", state));
        let mut joltages: Vec<Joltage> = vec![0; self.machine.get_joltages().len()];
        for (switch_id, num_presses) in state.iter().enumerate() {
            let switch = &self.machine.get_switches()[switch_id];
            for joltage_id in switch.iter() {
                joltages[*joltage_id] += *num_presses as u32;
            }
        }
        //logger().logn(&format!("[calculate_joltages] Joltages: {:?}", joltages));
        joltages
    }
    
    fn compare_vs_best_state(&self, current_state: &State2) -> CurrentVSDesired {
        if let Some(best_state) = &self.result {
            if current_state.button_presses() > best_state.button_presses() {
                CurrentVSDesired::Larger
            } else {
                CurrentVSDesired::Smaller
            }
        } else {
            // No result yet: any current state is better (smaller) than what found so far (nothing)
            CurrentVSDesired::Smaller
        }
    }
    
    fn compare_current_vs_desired_joltage(desired_joltages: &Vec<Joltage>, current_joltages: &Vec<Joltage>) -> CurrentVSDesired {
        let mut all_equal = true;
        for i in 0..desired_joltages.len() {
            if current_joltages[i] > desired_joltages[i] {
                return CurrentVSDesired::Larger;
            }
            if current_joltages[i] < desired_joltages[i] {
                all_equal = false;
            }
        }
        if all_equal {
            return CurrentVSDesired::Equal;
        }
        CurrentVSDesired::Smaller
    }
    
    
    fn update_state_if_better(&mut self, new_state: &State2) {
        let buttons_pressed_current = self.result.as_ref().map_or(u32::MAX, |best_state| best_state.button_presses());
        
        if new_state.button_presses() < buttons_pressed_current {
            logger().logn(&format!("********* Updating result to buttons pressed: {}", new_state.button_presses()));
            self.result = Some(new_state.clone());
        }
    }
    
    fn calculate_result_recursively(&mut self, state: State2) {
        if self.state_cache.contains(&state) {
            logger().logn(&format!("Cache hit: {:?}", state));
            return;
        } else {
            self.state_cache.insert(state.clone());
            logger().logn(&format!("Visited states: {:?}", self.state_cache.len()));
        }
        
        let local_min = &self.result.as_ref().map_or("None".to_string(), |best_state| {
            best_state.iter().sum::<u32>().to_string()
        });
        logger().logn(&format!("Current local minimum: {local_min}"));
        
        logger().logn(&format!("Current state: {}", &state.to_string()));
        let desired_joltages = self.machine.get_joltages();
        let current_joltages = self.calculate_joltages(&state);
        logger().logn(&format!("Current joltages: {:?}", current_joltages));
        logger().logn(&format!("Desired joltages: {:?}", desired_joltages));
        
        if matches!(self.compare_vs_best_state(&state), CurrentVSDesired::Larger) {
            logger().logn(&format!("Current state is larger than best state. No need to keep exploring"));
            return;
        }
        
        let current_vs_desired_joltages = Self::compare_current_vs_desired_joltage(desired_joltages, &current_joltages);
        
        match current_vs_desired_joltages {
            CurrentVSDesired::Equal => {
                // reached the result we wanted: don't continue down this path because next joltages will be too large
                logger().logn("Reached desired state");
                self.update_state_if_better(&state);
                return;     
            },
            CurrentVSDesired::Larger => {
                // Stop searching if pressing buttons doesn't make sense anymore
                logger().logn("Stopped search: current state won't lead to solution");
                return;             
            },
            _ => ()
        }

        // Explore pressing only switches that take us to state where one of the joltages is the desired
        let switch_presses_to_desired = current_joltages.iter()
            .zip(desired_joltages.iter())
            .map(|(current, desired)| desired - current)
            .collect::<Vec<Joltage>>();
        logger().logn(&format!("\tMissing presses: {:?}", switch_presses_to_desired));
        let (position_to_reduce, switch_presses) = switch_presses_to_desired.iter().enumerate()
            .filter(|(_, dist)| **dist > 0) // filter out positions that have already a desired joltage
            .min_by(|d1, d2| d1.1.cmp(d2.1)).unwrap();
        
        logger().logn(&format!("\tNext position, to press {} times, is {}", switch_presses, position_to_reduce));
        let valid_switch_ids = self.machine.get_switches().iter()
            .enumerate()
            .filter(|(_, switch)| {
                // Only switches that can reduce selected distance 
                switch.contains(position_to_reduce)
            })
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        
        let switch_combinations = self.composition_calculator.sums_of_n_in_k_parts(*switch_presses, valid_switch_ids.len());
        logger().logn(&format!("\tWill attempt to press {} times using the {} switches: {:?}", switch_presses, valid_switch_ids.len(),valid_switch_ids.iter().map(|id| &self.machine.get_switches()[*id]).collect::<Vec<&Switch>>()));
        logger().logn(&format!("\tCombinations possible: {}", switch_combinations.len()));
        
        for combination in switch_combinations {
            let presses_per_switch_id = combination.iter().enumerate()
                .map(|(pos, num_presses)| (valid_switch_ids[pos], *num_presses))
                .collect::<Vec<(usize, u32)>>();
            logger().logn(&format!("\tFrom state {}, trying combination: {:?}", state.to_string(), presses_per_switch_id));
            let new_state = state.new_state_by_pressing_many_buttons(&presses_per_switch_id);
            self.calculate_result_recursively(new_state);
        }
    }
    
    pub fn calculate_result(&mut self) -> MachineCalculatorResult2 {
        let initial_state = State2::new(self.machine.get_switches().len());
        self.calculate_result_recursively(initial_state);
        
        let result = self.result.as_ref().expect("Could not find a result for part 2");
        
        MachineCalculatorResult2 { 
            desired_joltages: self.machine.get_joltages().clone(), 
            result: result.clone(),
            switches: self.machine.get_switches().clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State2(Vec<u32>);

impl State2 {
    pub fn new(size: usize) -> Self {
        State2(vec![0; size])
    }
    
    pub fn to_string(&self) -> String {
        self.0.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &u32> {
        self.0.iter()
    }
    
    pub fn get(&self, i: &usize) -> Option<&u32> {
        self.0.get(*i)
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    pub fn button_presses(&self) -> u32 {
        self.0.iter().sum()
    }
    
    pub fn insert(&mut self, pos: usize, val: u32) {
        self.0[pos] = val;
    }
    
    fn new_state_by_clicking_times(&self, switch_id_to_try: usize, num_presses: u32) -> State2 {
        let mut new_state = self.clone();
        let current_attempts_switch = new_state.get(&switch_id_to_try).unwrap();
        new_state.insert(switch_id_to_try, *current_attempts_switch + num_presses);
        new_state
    }
    
    fn new_state_by_pressing_many_buttons(&self, presses_per_switch: &Vec<(usize, u32)>) -> State2 {
        logger().logn(&format!("\t\t[new_state_by_pressing_many_buttons] current state: {}", self.to_string()));
        let mut new_state = self.clone();
        for (switch_id_to_try, num_presses) in presses_per_switch {
            new_state = new_state.new_state_by_clicking_times(*switch_id_to_try, *num_presses);
        }
        logger().logn(&format!("\t\t[new_state_by_pressing_many_buttons] new state: {}", new_state.to_string()));
        new_state
    }
}

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
        for (switch_id, num_presses) in self.result.iter().enumerate() {
            let switch = self.switches.get(switch_id).unwrap();
            println!("\tswitch: {}", switch);
            println!("\tpressed times: {}", num_presses);
            total_presses += *num_presses;
        }
        println!("\ttotal pressed times: {}", total_presses);
        println!("+++++");
    }
    
    pub fn num_presses(&self) -> u32 {
        self.result.button_presses()
    }
}
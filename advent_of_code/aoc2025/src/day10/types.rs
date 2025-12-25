use std::fmt::Display;

pub type Joltage = u32;

#[derive(Clone, Debug)]
pub struct Switch(Vec<usize>);

impl Switch {
    pub fn new() -> Self {
        Switch(Vec::new())
    }
    pub fn from(vec: Vec<usize>) -> Self {
        Switch(vec)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn push(&mut self, switch: usize) {
        self.0.push(switch);
    }
    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.0.iter()
    }
}

impl Display for Switch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char_repr = &self.0.iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "({})", char_repr)
    }
}

#[derive(Clone)]
pub struct SwitchesPressed(Vec<Switch>);

impl SwitchesPressed {
    pub fn new() -> Self {
        SwitchesPressed(Vec::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn push(&mut self, switch: Switch) {
        self.0.push(switch);
    }
}

impl Display for SwitchesPressed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_repr = &self.0.iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "Switches {{ {} }}", str_repr)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct State(Vec<bool>);

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char_repr = &self.0.iter().map(|light: &bool| {
            match light {
                true => '#',
                false => '.',
            }
        }).collect::<String>();
        write!(f, "[{}]", char_repr)
    }
}

impl State {
    pub fn new(size: usize) -> Self {
        State(vec![false; size])
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    pub fn from(other_switch: &Self) -> Self {
        State::from_vec(other_switch.0.clone())
    }
    pub fn from_vec(lights: Vec<bool>) -> Self {
        State(lights)
    }
    
    pub fn is_all_off(&self) -> bool {
        self.0.iter().all(|light| !light)
    }

    pub fn apply_switch(&self, switch: &Switch) -> State {
        let mut new_state = Self::from(&self);
        for s in switch.iter() {
            new_state.bit_switch(*s);
        }
        new_state
    }
    
    fn bit_switch(&mut self, index: usize) {
        self.0[index] = !self.0[index];
    }
}

pub struct Machine {
    desired_state: State,
    switches: Vec<Switch>,
}

impl Machine {
    pub fn new(desired_state: State, _joltages: Vec<Joltage>, switches: Vec<Switch>) -> Self {
        Machine {
            desired_state,
            switches,
        }
    }
    
    pub fn get_desired_state(&self) -> &State {
        &self.desired_state
    }
    
    pub fn get_switches(&self) -> &Vec<Switch> {
        &self.switches
    }

}

#[cfg(test)]
mod tests {
    use super::{State, Switch};

    #[test]
    fn apply_switch_nothing() {
        let state = State::from_vec(vec![true, true, false, false]);
        let switch = Switch::from(vec![]);
        let new_state = state.apply_switch(&switch);
        
        println!("Original State:");
        println!("{}", state);
        println!("New State:");
        println!("{}", new_state);
        assert_eq!(state, new_state);
    }
    
    fn apply_switch_simple_middle() {
        let state = State::from_vec(vec![false, true, true, false, true]);
        let switch = Switch::from(vec![1, 2, 3, 4]);
        let new_state = state.apply_switch(&switch);
        
        println!("Original State:");
        println!("{}", state);
        println!("New State:");
        println!("{}", new_state);
        assert_eq!(new_state, State::from_vec(vec![false, false, false, true, false]));
    }
}
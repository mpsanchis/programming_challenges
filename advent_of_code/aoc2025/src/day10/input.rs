use std::fs;
use crate::day10::types::State;

use super::types::{Joltage, Machine, Switch};

fn parse_lights(section: &str) -> Vec<bool> {
    let mut lights = Vec::new();
    for char in section[1..(section.len()-1)].chars() {
        if char == '.' {
            lights.push(false);
        } else if char == '#' {
            lights.push(true);
        } else {
            panic!("Could not parse char '{char}' as a light");
        }
    }
    lights
}
fn parse_switch(section: &str) -> Switch {
    let mut switch = Switch::new();
    for num in section.chars()
        .filter_map(|c| c.to_digit(10))
    {
        switch.push(num as usize);
    }
    switch
}
fn parse_joltages(section: &str) -> Vec<Joltage> {
    let mut joltages = Vec::new();
    let section_nums = section[1..section.len()-1].split(',');
    for num_str in section_nums {
        let num = num_str.parse().expect("Could not parse number {num_str}");
        joltages.push(num);
    }
    joltages
}

pub fn parse_input() -> Vec<Machine> {
    let input = fs::read_to_string(file!().replace(".rs", ".txt"))
        .expect("could not read input");
    
    let mut machines: Vec<Machine> = Vec::new();
    
    for line in input.lines() {
        let mut desired_lights: Vec<bool> = Vec::new();
        let mut switches: Vec<Switch> = Vec::new();
        let mut joltages: Vec<Joltage> = Vec::new();
        
        let input_sections = line.split(' ');
        for section in input_sections {
            match section.chars().next().unwrap() {
                '[' => desired_lights.extend(parse_lights(section)),
                '(' => switches.push(parse_switch(section)),
                '{' => joltages.extend(parse_joltages(section)),
                _ => panic!("Could not parse section: {}", section),
            }
        }
        machines.push(Machine::new(
            State::from_vec(desired_lights), 
            joltages, 
            switches
        ));
    }
    machines
}
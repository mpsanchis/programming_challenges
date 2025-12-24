use crate::util::{Part};

mod types;
mod input;
mod solver;

use input::parse_input;
use types::Region;
use solver::{area_solver};

fn main_1(input_list: Vec<(Region, Vec<u16>)>) {
    println!("Number of regions: {}", input_list.len());
    
    let mut regions_can_fit_area = 0;
    let mut regions_can_fit_area_coarse = 0;
    
    for (region, presents) in input_list {
        if area_solver::can_presents_fit_in_region_coarse(&region, &presents) {
            regions_can_fit_area_coarse += 1;
        }
        if area_solver::can_presents_fit_in_region(&region, &presents) {
            regions_can_fit_area += 1;
        }
    }
    
    println!("Regions that can fit all figures, considering all have area of 9: {regions_can_fit_area_coarse}");
    println!("Regions that can fit all figures, considering their area and shape: {regions_can_fit_area}");
}

pub fn main(part: &Part) {
    let input = parse_input();
    match part {
        Part::One => main_1(input),
        _ => ()
    }
}
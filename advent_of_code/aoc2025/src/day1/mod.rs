mod util;
use crate::util::{Part, logger};

pub use util::parse_input;

fn main_pt1() {
    let logger = logger();

    let mut hits_zero = 0;
    let mut position = 50;
    let input = parse_input();

    // println!("input: {input:?}");

    for rotation in input.iter() {
        logger.log(&format!("position = {position}"));
        logger.log(&format!("rotation = {rotation}"));
        position = (position + rotation).rem_euclid(100);
        logger.log(&format!("new position = {position}"));
        if position == 0 {
            hits_zero += 1;
        }
    }

    println!("Hits zero: {hits_zero}");
}


fn stops_at_or_crosses_zero(position: i16, new_position: i16, rotation: i16) -> bool {
    if new_position == 0 {
        return true;
    }
    if position == 0 {
        return false;
    }

    if rotation < 0 {
        new_position > position
    } else {
        new_position < position
    }
}

fn main_pt2() {
    let logger = logger();

    let mut crosses_zero_total = 0;
    let mut position = 50;
    let input = parse_input();

    for rotation in input.iter() {
        logger.log("===");
        logger.log(&format!("crosses zero total = {crosses_zero_total}"));
        logger.log(&format!("position = {position}"));
        logger.log(&format!("rotation = {rotation}"));
        let destination = position + rotation;
        let new_position = destination.rem_euclid(100);

        // Calculate number of zero crossings
        let min_loops = rotation.abs().div_euclid(100);
        let mut crosses_zero = min_loops;
        crosses_zero += stops_at_or_crosses_zero(position, new_position, *rotation) as i16;

        position = new_position;
        logger.log("=");
        logger.log(&format!("new position = {destination} =eq= {position}"));
        crosses_zero_total += crosses_zero;
        logger.log(&format!("crosses zero = {crosses_zero}"));
        logger.log(&format!("crosses zero total = {crosses_zero_total}"));
    }

    println!("Crosses zero (total): {crosses_zero_total}");
}

pub fn main(part: &Part) {
    match part {
        Part::One => main_pt1(),
        Part::Two => main_pt2(),
    }
}

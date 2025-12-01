mod util;

pub use util::parse_input;

pub fn main_pt1() {
    let mut hits_zero = 0;
    let mut position = 50;
    let input = parse_input();

    // println!("input: {input:?}");

    for rotation in input.iter() {
        println!("position = {position}");
        println!("rotation = {rotation}");
        position = (position + rotation).rem_euclid(100);
        println!("new position = {position}");
        if position == 0 {
            hits_zero += 1;
        }
    }

    println!("Hits zero: {}", hits_zero);
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

pub fn main_pt2() {
    let mut crosses_zero_total = 0;
    let mut position = 50;
    let input = parse_input();

    for rotation in input.iter() {
        println!("===");
        println!("crosses zero total = {crosses_zero_total}");
        println!("position = {position}");
        println!("rotation = {rotation}");
        let destination = position + rotation;
        let new_position = destination.rem_euclid(100);

        // Calculate number of zero crossings
        let min_loops = rotation.abs().div_euclid(100);
        let mut crosses_zero = min_loops;
        crosses_zero += stops_at_or_crosses_zero(position, new_position, *rotation) as i16;

        position = new_position;
        println!("=");
        println!("new position = {destination} =eq= {position}");
        crosses_zero_total += crosses_zero;
        println!("crosses zero = {crosses_zero}");
        println!("crosses zero total = {crosses_zero_total}");
    }

    println!("Crosses zero: {}", crosses_zero_total);
}
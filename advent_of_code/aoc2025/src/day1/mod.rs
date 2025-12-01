mod util;

pub use util::parse_input;

pub fn main() {
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
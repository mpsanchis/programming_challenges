use crate::util::{logger};

pub fn is_invalid_id(id_: u64) -> bool {
    let id = id_.to_string();
    let num_chars = id.len();
    if num_chars % 2 != 0 {
        return false;
    }

    let (half_1, half_2) = id.split_at(num_chars/2);
    half_1 == half_2
}

pub fn pattern_window_lengths(id_: u64) -> Vec<usize> {
    let mut window_lengths = Vec::new();
    let id = id_.to_string();

    for window_length in 1..=id.len()/2 {
        if id.len() % window_length == 0 {
            window_lengths.push(window_length);
        }
    }
    window_lengths
}

fn id_matches_pattern(id: &String, pattern_window_length: &usize) -> bool {
    let logger = logger();

    let pattern_window = id.get(0..*pattern_window_length).unwrap();
    logger.logn(&format!("Pattern window: {}", pattern_window));

    for step in 0..(id.len() / pattern_window_length) {
        let id_slice = id.get(step * pattern_window_length..(step + 1) * pattern_window_length).unwrap();
        logger.logn(&format!("ID slice: {}", id_slice));
        if id_slice != pattern_window {
            logger.logn(&format!("ID slice does not match pattern window"));
            return false;
        }
    }
    true
}

pub fn is_invalid_id_2(id_: u64) -> bool {
    let logger = logger();

    let id = id_.to_string();
    logger.logn(&format!("---\nID: {}\n---", id));

    let pattern_window_lengths = pattern_window_lengths(id_);
    logger.logn(&format!("Pattern window lengths: {:?}", pattern_window_lengths));

    for pattern_window_length in pattern_window_lengths.iter() {
        if id_matches_pattern(&id, pattern_window_length) {
            return true;
        }
    }
    false
}
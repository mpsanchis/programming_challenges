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

pub fn is_invalid_id_2(id_: u64) -> bool {
    let logger = logger();

    let id = id_.to_string();
    logger.log(&format!("---\nID: {}\n---", id));

    let mut pattern = id.as_bytes().get(0..1).unwrap();

    for (index, byte) in id.as_bytes().iter().enumerate() {
        // logger.log(&format!("-\nIndex: {}, Digit: {}", index, *byte as char));
        // logger.log(&format!("Pattern: {} ({})", String::from_utf8(Vec::from(pattern)).unwrap(), pattern.len()));
        // check if pattern got too big and finish early
        if pattern.len() > id.len() / 2 {
            logger.log("Pattern too big. No pattern found");
            return false;
        }

        if pattern[index % pattern.len()] == *byte {
            // We match the pattern, so continue with the same pattern
            // logger.log(&format!("Matched pattern: {}", String::from_utf8(Vec::from(pattern)).unwrap()));
            continue;
        } else {
            // If we reached the end, we're done: no repeated pattern found
            if index == id.len() - 1 {
                // logger.log("Reached the end. No pattern found");
                return false;
            }
            // If number is not in pattern, then we should increase the pattern window
            pattern = id.as_bytes().get(0..index+1).unwrap();
            // logger.log(&format!("Increased pattern to: {}", String::from_utf8(Vec::from(pattern)).unwrap()));
        }
    }
    true
}
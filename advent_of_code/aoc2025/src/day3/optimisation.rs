use crate::util::{logger};

pub fn max_joltage(bank: &[u8]) -> u16 {
    let logger = logger();

    let mut max_joltage: u16 = 0;
    for i in 0..bank.len()-1 {
        let tenth_digit = bank[i];
        for j in i+1..bank.len() {
            let unit_digit = bank[j];
            let joltage = (tenth_digit as u16) * 10 + (unit_digit as u16);
            logger.log(&format!("Saw joltage: {joltage}"));
            if joltage > max_joltage {
                logger.log(&format!("Max joltage updated to {joltage}"));
                max_joltage = joltage;
            }
        }
    }
    max_joltage
}
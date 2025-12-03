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

pub fn max_joltage_2(bank: &[u8]) -> u16 {
    let logger = logger();

    let (pos_max_tenths, max_tenths) = bank.get(0..bank.len()-1).unwrap().iter().enumerate().min_by(|x,y| {
        // hack to get the FIRST max, otherwise 'max_by' returns the LAST max
        let x_neg = -(*x.1 as i32);
        let y_neg = -(*y.1 as i32);
        x_neg.cmp(&y_neg)
    }).unwrap();
    logger.log(&format!("Max joltage tenths: {max_tenths} (position: {}/{}))", pos_max_tenths, bank.len()-1));
    let max_units = bank.get(pos_max_tenths+1..bank.len()).unwrap().iter().max().unwrap();

    let joltage = (*max_tenths as u16) * 10 + (*max_units as u16);
    logger.log(&format!("Max joltage updated to {joltage}"));
    joltage
}
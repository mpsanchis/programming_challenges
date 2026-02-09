
fn next_alphanumeric(s_chars: &[u8], i: isize) -> usize {
    let mut i_next = i as usize;
    let mut first_iter = i == -1;
    loop {
        i_next = if first_iter { 
            first_iter = false;
            0 
        } else { i_next + 1 };

        if i_next > s_chars.len() - 1 {
            return s_chars.len() - 1;
        }

        let char_next = s_chars[i_next] as char;

        if char_next.is_alphanumeric() {
            return i_next;
        }
    }
}

fn prev_alphanumeric(s_chars: &[u8], j: usize) -> usize {
    let mut j_prev = j; 
    loop {
        j_prev = if j_prev > 0 { j_prev - 1 } else { 0 };

        if j_prev == 0 {
           return 0; 
        }

        let char_prev = s_chars[j_prev] as char;

        if char_prev.is_alphanumeric() {
            return j_prev;
        }
    }
}

pub fn is_palindrome(s: String) -> bool {
    let s_chars = s.as_bytes();

    if s_chars.len() == 0 { 
        return true;
    }

    let mut i = next_alphanumeric(s_chars, -1);
    let mut j = prev_alphanumeric(s_chars, s.len());

    while i < j {
        // println!("Checking i={}, j={}", i, j);
        if s_chars[i].to_ascii_lowercase() != s_chars[j].to_ascii_lowercase() {
            return false;
        }
        i = next_alphanumeric(s_chars, i as isize);
        // println!("i next: {}", i);
        j = prev_alphanumeric(s_chars, j);
        // println!("j next: {}", j);
    }

    true
}

fn main() {
    /*
    println!(
        "{}", is_palindrome("A man, a plan, a canal: Panama".to_string())
    );
    println!(
        "{}", is_palindrome("race a car".to_string())
    );
    */
    println!(
        "{}", is_palindrome(" ".to_string())
    );
}

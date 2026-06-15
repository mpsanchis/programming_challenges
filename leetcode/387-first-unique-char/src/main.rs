use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut letter_freq: HashMap<char, usize> = HashMap::new();

        // First pass: keep letter frequency in the string
        for letter in s.chars() {
            letter_freq
                .entry(letter)
                .and_modify(|freq| *freq += 1)
                .or_insert(1);
        }

        if letter_freq.is_empty() {
            return -1;
        }

        // Second pass: get letter with smallest position
        for (position, letter) in s.chars().enumerate() {
            if letter_freq
                .get(&letter)
                .is_some_and(|frequency| *frequency == 1)
            {
                return position as i32;
            }
        }

        -1
    }
}

fn test(s: &str, expected_out: i32) {
    let out = Solution::first_uniq_char(String::from(s));
    println!("{} --> {}", s, out);
    assert_eq!(expected_out, out);
}

fn main() {
    // Test cases
    test("leetcode", 0);
    test("loveleetcode", 2);
    test("aabb", -1);
    test("aadadaad", -1);
}

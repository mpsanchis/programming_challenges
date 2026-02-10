use std::collections::{HashMap};

struct Solution;

impl Solution {
    fn to_canonical_form(s: &String) -> [u8; 26] {
        let mut letter_counts = [0u8; 26];
        for b in s.bytes() {
            letter_counts[(b - b'a') as usize] += 1;
        }
        letter_counts
    }
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_groups: HashMap<[u8; 26], usize> = HashMap::new();
        let mut result: Vec<Vec<String>> = Vec::new();

        for s in strs {
            let s_canonical = Solution::to_canonical_form(&s);
            if let Some(str_id) = anagram_groups.get(&s_canonical) {
                result[*str_id].push(s);
            } else {
                result.push(vec![s]);
                anagram_groups.insert(s_canonical, result.len() - 1);
            }
        }

        result
    }
}

fn main() {
    let strs = vec!["eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()];
    // let strs = vec!["".to_string(), "".to_string()];
    let res = Solution::group_anagrams(strs);
    println!("res: {:?}", res);
}

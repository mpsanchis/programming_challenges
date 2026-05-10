struct Solution;

impl Solution {
    fn is_opening(c: char) -> bool {
        c == '(' || c == '[' || c == '{'
    }
    fn closing(c: char) -> char {
        if c == '(' {
            return ')';
        } else if c == '{' {
            return '}';
        } else {
            return ']';
        }
    }
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            // println!("Getting: '{}'", c);
            if Solution::is_opening(c) {
                // println!("Pushing: '{}'", c);
                stack.push(c);
            } else {
                // Problem ensures input is only {}()[] --> must be closing
                if let Some(top_char) = stack.pop() {
                    if c == Solution::closing(top_char) {
                        // println!("'{c}' is closing the top: '{top_char}");
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    // No elements in the stack --> we cannot close anything
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

fn evaluate(s: &str) {
    let sol = Solution::is_valid(s.to_string());
    println!("'{}' {} valid", s, if sol { "IS" } else { "IS NOT" });
}

fn main() {
    let s = "()";
    evaluate(s);

    let s = "()[]{}";
    evaluate(s);

    let s = "(]";
    evaluate(s);

    let s = "([])";
    evaluate(s);

    let s = "([)]";
    evaluate(s);
}

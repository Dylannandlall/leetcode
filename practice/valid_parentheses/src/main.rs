/* 
Author: Dylan Nandlall
Problem: Valid Parantheses
12/16/22
 */

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut string_vec: Vec<char> = Vec::new();

        for c in s.chars() {
            if c == '{' {
                string_vec.insert(0, '}');
            } else if c == '[' {
                string_vec.insert(0, ']');
            } else if c == '(' {
                string_vec.insert(0, ')');
            } else if string_vec.is_empty() || string_vec.remove(0) != c {
                return false;
            }
        }
        return string_vec.is_empty();
    }
}
fn main() {
    let result = Solution::is_valid("[".to_string());
    println!("{}", result);
}

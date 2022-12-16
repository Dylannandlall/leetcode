/*
Author: Dylan Nandlall
Problem: Palindrome Number
12/16/22
 */

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let y = x.to_string();
        let z: String = x.to_string().chars().rev().collect();
        return y == z;
    }
}

fn main() {
    let result: bool = Solution::is_palindrome(1212121);
    println!("{}", result);
}

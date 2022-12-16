/*
Author: Dylan Nandlall
Problem: Roman To Integer
12/16/22
*/


struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut value: i32 = 0;
        let mut last = 0;
        
        for c in s.chars() {
            if last >= Self::converter(c) || last == 0 {
                value = value + last;
                last = Self::converter(c);
            } else {
                value = value + (Self::converter(c) - last);
                last = 0;
            }
        }
        return value + last;
    }

    pub fn converter(c: char) -> i32 {
        match c {
            'I'=>1,
            'V'=>5,
            'X'=>10,
            'L'=>50,
            'C'=>100,
            'D'=>500,
            'M'=>1000,
            _=>0,
        }
    }
}

fn main() {
    let result = Solution::roman_to_int("III".to_string());
    println!("III: {}", result);

    let result = Solution::roman_to_int("LVIII".to_string());
    println!("LVIII: {}", result);

    let result = Solution::roman_to_int("MCMXCIV".to_string());
    println!("MCMXCIV: {}", result);
}

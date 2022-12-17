/*
Author: Dylan Nandlall
Problem: Remove Element
12/17/22
 */

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = nums.len() as i32;
        nums.retain(|x| *x != val);
        let difference = count - nums.len() as i32;

        for i in 0..difference as i32 {
            nums.push(-1);
        }
        return count-difference;
    }
}
fn main() {
    let mut nums = vec![3,2,2,3];
    let result = Solution::remove_element(&mut nums, 3);
    println!("{:?}", result);

    let mut nums = vec![0,1,2,2,3,0,4,2];
    let result = Solution::remove_element(&mut nums, 2);
    println!("{:?}", result);
    
}

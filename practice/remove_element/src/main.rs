/*
Author: Dylan Nandlall
Problem: Remove Element
12/17/22
 */

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = nums.len();
        let mut i = 0;

        while i < count {
            if nums[i] == val {
                nums.remove(i);
                nums.push(-1);
                count -= 1;
                continue;
            } 
            i += 1;
        }
        return count as i32;
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

/*
Author: Dylan Nandlall
Two Sum Problem
12/16/22
 */
struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            let difference = target - nums[i];
            for j in (i+1)..nums.len() {
                if nums[j] == difference {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![];
    }
}
fn main() {
    let nums = vec![2,7,11,15];
    let target = 9;
    let result: Vec<i32> = Solution::two_sum(nums, target);
    println!("{:?}", result);

    let nums = vec![3,2,4];
    let target = 6;
    let result: Vec<i32> = Solution::two_sum(nums, target);
    println!("{:?}", result);

    let nums = vec![3,3];
    let target = 6;
    let result: Vec<i32> = Solution::two_sum(nums, target);
    println!("{:?}", result);
}

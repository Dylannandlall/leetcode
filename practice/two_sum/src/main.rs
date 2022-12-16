/*
Author: Dylan Nandlall
Two Sum Problem
12/16/22
 */

use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            if map.contains_key(&(target - nums[i])) {
                return vec![i as i32, *map.get(&(target - nums[i])).unwrap()]
            }
            map.insert(nums[i], i as i32);
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

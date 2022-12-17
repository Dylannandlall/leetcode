/*
Author: Dylan Nandlall
Problem: Search Insert Position
12/17/22
*/

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if target < nums[0] {
            return 0;
        } 
        if target > nums[nums.len()-1] {
            return (nums.len()) as i32;
        }

        let mut low = 0;
        let mut high = nums.len() - 1;
        while low <= high {
            let mid = Self::floor_division_by_2(low, high);
            if target == nums[mid] {
                return mid as i32;
            }
            if target < nums[mid] {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        return low as i32;
    }

    pub fn floor_division_by_2(l: usize, h: usize) -> usize {
        let val = ((l+h)/2) as f64;
        return val.floor() as usize;
    }
}
fn main() {
    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 5);
    println!("{}", result);

    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 2);
    println!("{}", result);

    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 7);
    println!("{}", result);

    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 0);
    println!("{}", result);

    let nums = vec![1,3];
    let result = Solution::search_insert(nums, 1);
    println!("{}", result);
}
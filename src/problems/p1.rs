/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order.
*/

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut already_found: HashMap<i32, i32> = HashMap::new();

        let mut result: Vec<i32> = vec![];

        for i in 0..nums.len() {
            let current_number = nums[i];
            let remainder = target - current_number;


            if already_found.contains_key(&remainder) {
                let already_found_index = already_found.get(&remainder).unwrap();
                result.push(*already_found_index);
                result.push(i as i32);
            }
            already_found.insert(current_number, i as i32);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_two_sum() {
        assert!(true);
    }
}
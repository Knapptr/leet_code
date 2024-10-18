/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
*/
use crate::solution::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut counter = HashMap::new();
        // iterate through array and mark index of each value
        // if the nesc value in array already exists, return the two indicies
        for (index, num) in nums.iter().enumerate() {
            let nesc_value = target - num;
            match counter.get(&nesc_value) {
                Some(index2) => return vec![index as i32, *index2 as i32],
                None => {
                    counter.insert(num, index);
                }
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let (nums, target) = (vec![2, 7, 11, 15], 9);
    let mut expected = vec![0, 1];
    expected.sort();
    let mut solution = Solution::two_sum(nums, target);
    solution.sort();
    assert_eq!(solution, expected);
}
#[test]
fn example_two() {
    let (nums, target) = (vec![3, 2, 4], 6);
    let mut expected = vec![2, 1];
    expected.sort();
    let mut solution = Solution::two_sum(nums, target);
    solution.sort();
    assert_eq!(solution, expected);
}
#[test]
fn example_three() {
    let (nums, target) = (vec![3, 3], 6);
    let mut expected = vec![0, 1];
    expected.sort();
    let mut solution = Solution::two_sum(nums, target);
    solution.sort();
    assert_eq!(solution, expected);
}

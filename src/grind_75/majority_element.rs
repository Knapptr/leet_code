use std::collections::HashMap;

use crate::solution::Solution;

/*
Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.
*/

impl Solution {
    fn majority_element(nums: Vec<i32>) -> i32 {
        // Optimized for space
        let mut count = 0;
        let mut element = 69;
        for num in nums {
            if count == 0 {
                element = num
            }
            if num == element {
                count += 1
            } else {
                count -= 1
            }
        }
        element
        /* Using Hash Map
           let mut counter = HashMap::new();
        for num in &nums {
            let entry = counter.entry(num).or_insert(0);
            *entry += 1;
            if *entry > nums.len() as i32 / 2 {
                return *num;
            }
        }
        unreachable!();
        */
    }
}
#[cfg(test)]
#[test]
fn example_one() {
    let nums = vec![3, 2, 3];
    let expected = 3;
    assert_eq!(Solution::majority_element(nums), expected);
}
#[test]
fn example_two() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let expected = 2;
    assert_eq!(Solution::majority_element(nums), expected);
}

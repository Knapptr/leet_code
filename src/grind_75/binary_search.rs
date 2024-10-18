use crate::solution::Solution;
/*Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

You must write an algorithm with O(log n) runtime complexity.*/

impl Solution {
    fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut offset = 0;
        let mut slice = &nums[0..];

        loop {
            // check halfway
            let mid_index = slice.len() / 2;
            if mid_index >= slice.len() {
                return -1;
            }
            // Handle an empty set
            if slice.len() == 0 {
                return -1;
            } else if slice[mid_index] == target {
                // Handle if the midpoint is the value
                return (mid_index + offset) as i32;
            } else if slice[mid_index] > target {
                // Handle if the midpoint is greater
                slice = &slice[0..mid_index]
            } else if slice[mid_index] < target {
                // Handle if the midpoint is less
                slice = &slice[mid_index + 1..];
                offset += mid_index + 1;
            }
        }
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    let expected = 4;

    assert_eq!(Solution::search(nums, target), expected);
}

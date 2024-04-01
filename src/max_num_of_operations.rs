// Given an array of integers called nums, you can perform the following operation while nums contains at least 2 elements:

// Choose the first two elements of nums and delete them.
// The score of the operation is the sum of the deleted elements.

// Your task is to find the maximum number of operations that can be performed, such that all operations have the same score.

pub struct Solution;

impl Solution {
    fn max_operations(mut nums: Vec<i32>) -> i32 {
        let mut score = 0;
        let mut count = 0;
        while nums.len() >= 2 {
            // sum first 2 elements in vec
            let sum = nums[0] + nums[1];
            if score == 0 {
                score = sum;
            } else {
                if score != sum {
                    break;
                }
                count += 1;
                nums = nums[2..].to_vec();
            }
        }
        count
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let nums = vec![3, 2, 1, 4, 5];
    let expected = 2;

    let result = Solution::max_operations(nums);

    assert_eq!(expected, result)
}

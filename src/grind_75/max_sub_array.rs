use crate::solution::Solution;
impl Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        /*DP Table was overkill. This may be done in O(n) time (not the O(n2) the table uses)

        A similar strategy to the solution used in "Best Time to Buy Stock" may be used.

        Keep a "best_sum" variable, and a "current_sum" var. Increment the current_sum to  current_sum.max(current_sum + n) This will keep the current sum moving around as a window. Then update the best_sum as a tracker
        */

        /*let len = nums.len();
        let mut dp_table = vec![vec![0; len]; 2];
        let mut max = nums[0];
        // add value of number to value of dp[y-1][x+1] (or 0)
        for (i_y, num) in nums.iter().enumerate() {
            for i_x in 0..len {
                if i_y == 0 || i_x == len - 1 {
                    dp_table[i_y % 2][i_x] = *num
                } else {
                    dp_table[i_y % 2][i_x] = num + dp_table[(i_y - 1) % 2][i_x + 1]
                }
                max = std::cmp::max(dp_table[i_y % 2][i_x], max);
            }
        }
        // return the largest sum
        max
        */
        let mut current_sum = 0;
        let mut best_sum = i32::MIN;
        for n in nums {
            current_sum = n.max(n + current_sum);
            best_sum = best_sum.max(current_sum);
        }
        best_sum
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let expected = 6;
    assert_eq!(Solution::max_sub_array(nums), expected);
}

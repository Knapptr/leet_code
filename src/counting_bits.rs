// Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.

pub struct Solution;
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        // not sure how do do this outright mathematically, though surely there is an easy way
        // instead, I'll convert to a string and count- which seems silly
        let mut answer: Vec<i32> = Vec::new();

        for x in 0..=n {
            let mut count = 0;
            // conver to binary digit
            let bin_x_str = format!["{:b}", x];
            for char in bin_x_str.chars() {
                if char == '1' {
                    count += 1
                }
            }
            answer.push(count);
        }
        answer
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let n = 2;
    let expected = vec![0, 1, 1];

    let result = Solution::count_bits(n);

    assert_eq!(result, expected);
}

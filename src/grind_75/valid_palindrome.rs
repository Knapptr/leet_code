use std::ascii::AsciiExt;

use crate::solution::Solution;

/*A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.*/

impl Solution {
    fn is_palindrome(s: String) -> bool {
        let chars = s.as_bytes();
        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            if !chars[left].to_ascii_lowercase().is_ascii_alphanumeric() {
                left += 1;
                continue;
            }
            if !chars[right].to_ascii_lowercase().is_ascii_alphanumeric() {
                right -= 1;
                continue;
            }
            if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
                return false;
            }
            left += 1;
            right -= 1;
        }
        return true;
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let s = "A man a plan a canal panama.".to_string();
    let expected = true;
    assert_eq!(Solution::is_palindrome(s), expected);
}

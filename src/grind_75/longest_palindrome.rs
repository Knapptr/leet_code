/*Given a string s which consists of lowercase or uppercase letters, return the length of the longest
palindrome
 that can be built with those letters.

Letters are case sensitive, for example, "Aa" is not considered a palindrome.*/

use std::collections::HashMap;

use crate::solution::Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        // count the chars in the string
        let mut counter = HashMap::new();
        for char in s.chars() {
            let entry = counter.entry(char).or_insert(0);
            *entry += 1;
        }
        // add all even numbers, and longest odd number
        let mut sum = 0;
        let mut has_odd = false;
        for value in counter.values() {
            if value % 2 == 0 {
                sum += value
            } else {
                sum += value - 1;
                has_odd = true;
            }
        }
        match has_odd {
            true => sum + 1,
            false => sum,
        }
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let string = String::from("abccccdd");
    let expected = 7;
    assert_eq!(Solution::longest_palindrome(string), expected);
}

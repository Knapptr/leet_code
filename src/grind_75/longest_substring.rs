/* Given a string s, return the longest palindromic substring in s. */

use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::solution::Solution;

impl Solution {
    pub fn longest_substring(s: String) -> String {
        // DP Table
        // x b a b a d
        // - 1 1 1 1 1
        // 1 1 1 1 1 1
        // 2 1 3 3 1 1
        // 3 1 3 3 1 1
        // 4 ...
        /////
        let chars: Vec<char> = s.chars().collect();
        // Odd Number (1 center)
        let mut dp_table = vec![vec![0; chars.len()]; chars.len()];
        for offset in 0..chars.len() {
            for (char_index, char) in chars.iter().enumerate() {
                if offset < char_index + 1 && char_index + offset < chars.len() {
                    let (index_left, index_right) = ((char_index - offset), (char_index + offset));
                    // compare chars
                    if chars[index_left] == chars[index_right] {
                        // if match, add value of prev iy in ix
                        dp_table[offset][char_index] += 1;
                        if offset > 0 {
                            dp_table[offset][char_index] += dp_table[offset - 1][char_index - 1]
                        }
                    }
                }
                print!("{:?}", dp_table[offset][char_index]);
            }
            print!("\n")
        }
        io::stdout().flush();
        todo!()
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let string = "babad".to_string();
    let expected = "bab";
    assert_eq!(Solution::longest_substring(string), expected);
}

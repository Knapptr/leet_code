// You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.

// Return the merged string.
//
//
pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        let mut new_string = String::new();

        loop {
            match (chars1.next(), chars2.next()) {
                (Some(c1), Some(c2)) => {
                    new_string.push(c1);
                    new_string.push(c2);
                }
                (Some(c), None) | (None, Some(c)) => {
                    new_string.push(c);
                }
                (None, None) => {
                    break;
                }
            };
        }
        new_string
    }
}

#[cfg(test)]
#[test]
fn the_empty_string() {
    let str1 = String::from("One");
    let str2 = String::from("");

    let expected = String::from("One");

    let result = Solution::merge_alternately(str1, str2);
    assert_eq!(expected, result);
}

#[test]
fn test_one() {
    let str1 = String::from("One");
    let str2 = String::from("Two");

    let expected = String::from("OTnweo");

    let result = Solution::merge_alternately(str1, str2);
    assert_eq!(expected, result);
}

#[test]
fn test_two() {
    let str1 = String::from("Ones");
    let str2 = String::from("Two");

    let expected = String::from("OTnweos");

    let result = Solution::merge_alternately(str1, str2);
    assert_eq!(expected, result);
}

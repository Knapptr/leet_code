use crate::solution::Solution;

/* Given two strings s and t, return true if t is an
anagram
of s, and false otherwise.*/

impl Solution {
    fn is_anagram(s: String, t: String) -> bool {
        // I have done this before and would normally use a hash map,
        // but I'm going to try to make this one as efficient as possible
        // by using an array (all chars are going to be lowercase ascii)
        let mut counter = [0; 26];

        // I think that counting in the iteration of bytes will save a (constant) iteration- for marginal gains!
        // count str 1
        let mut len_s = 0;
        let mut len_t = 0;
        for byte in s.as_bytes() {
            len_s += 1;
            counter[(byte - 'a' as u8) as usize] += 1;
        }
        // mutate for str 2
        for byte in t.as_bytes() {
            len_t += 1;
            if counter[(byte - 'a' as u8) as usize] == 0 {
                return false;
            } else {
                counter[(byte - 'a' as u8) as usize] -= 1
            }
        }
        return len_s == len_t;
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let str1 = "anagram".to_string();
    let str2 = "nagaram".to_string();
    let expected = true;

    assert_eq!(Solution::is_anagram(str1, str2), expected)
}

// There is a malfunctioning keyboard where some letter keys do not work. All other keys on the keyboard work properly.

// Given a string text of words separated by a single space (no leading or trailing spaces) and a string brokenLetters of all distinct letter keys that are broken, return the number of words in text you can fully type using this keyboard.

use std::collections::HashSet;
pub struct Solution;
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        // split text into words
        // search each word for any of the broken letters
        // count words with no matches
        let words = text.split(' ');
        let broken_char_set: HashSet<char> = broken_letters.chars().collect();

        let mut count = 0;

        for word in words {
            let mut pass = true;
            for char in word.chars() {
                if broken_char_set.contains(&char) {
                    pass = false;
                    break; // minor optimization- stop checking the current word if it has already been
                           // deemed untypeable
                }
            }
            if pass {
                count += 1
            }
        }
        count
    }
}

#[cfg(test)]
#[test]
fn test_one() {
    let text = "hello world".to_string();
    let broken_letters = "h".to_string();
    let count = Solution::can_be_typed_words(text, broken_letters);
    let expected = 1;

    assert_eq!(count, expected);
}

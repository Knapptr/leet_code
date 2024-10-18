// Given a binary string s, return the number of non-empty substrings that have the same number of 0's and 1's, and all the 0's and all the 1's in these substrings are grouped consecutively.

// Substrings that occur multiple times are counted the number of times they occur.
//
pub struct Solution;
fn is_one(char: char) -> bool {
    if char == '1' {
        true
    } else {
        false
    }
}
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        let mut first_set_start = 0;

        while first_set_start < chars.len() - 1 {
            let first_set_1 = is_one(chars[first_set_start]);
            let mut second_set_start = first_set_start;
            let mut second_set_end = second_set_start;
            // move second start until it hits a different number
            while second_set_start < chars.len() && is_one(chars[second_set_start]) == first_set_1 {
                second_set_start += 1;
                second_set_end = second_set_start;
            }

            // move second set end
            while second_set_end < chars.len() - 1
                && is_one(chars[second_set_end]) != first_set_1
                && second_set_end - second_set_start < second_set_start - first_set_start
            {
                second_set_end += 1;
            }

            if second_set_end > first_set_start
                && second_set_end < chars.len()
                && second_set_end - second_set_start == second_set_start - first_set_start
            {
                println!("{} - {}", first_set_start, second_set_end);
                println!(
                    "{}, {}",
                    second_set_end - second_set_start,
                    second_set_start - first_set_start
                );
                count += 1;
            }
            first_set_start += 1;
        }

        count
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let input = "00110011".to_string();

    // 00110011
    // 01234567

    let expected = 6;
    let result = Solution::count_binary_substrings(input);

    assert_eq!(expected, result);
}

#[test]
fn example_two() {
    let input = "10101".to_string();

    // 10101
    // 01234

    let expected = 4;
    let result = Solution::count_binary_substrings(input);

    assert_eq!(expected, result);
}

#[test]
fn failed_submission_one() {
    let input = "00110".to_string();

    println!("00110");
    println!("01234");

    let expected = 3;
    let result = Solution::count_binary_substrings(input);

    assert_eq!(expected, result);
}

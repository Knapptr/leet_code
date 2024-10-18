use std::collections::HashMap;

/*Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

    Open brackets must be closed by the same type of brackets.
    Open brackets must be closed in the correct order.
    Every close bracket has a corresponding open bracket of the same type.
*/
use crate::solution::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // Convert string to u8, as this will always be ascii (it is parens only)
        let chars = s.as_bytes();
        let pairs = [('(', ')'), ('[', ']'), ('{', '}')];
        let pair_map = HashMap::from(pairs);
        // use a stack, push open brackets onto stack, remove them when meeting a valid closing bracket
        // always look at the last element on the stack
        let mut stack = vec![];
        for char in chars {
            // check if character is open parens
            // if it is, add it to the stack if it is not, check that the last element on the stack is its mate
            if pair_map.contains_key(&(*char as char)) {
                stack.push(char);
            } else {
                match stack.last() {
                    None => return false,
                    Some(open) => {
                        if *pair_map.get(&(**open as char)).unwrap() == *char as char {
                            stack.pop();
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
        return stack.len() == 0;
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let input = "()".to_string();
    let expected = true;
    assert_eq!(Solution::is_valid(input), expected)
}
#[test]
fn example_two() {
    let input = "()[]{}".to_string();
    let expected = true;
    assert_eq!(Solution::is_valid(input), expected)
}
#[test]
fn example_three() {
    let input = "(]".to_string();
    let expected = false;
    assert_eq!(Solution::is_valid(input), expected)
}
#[test]
fn example_four() {
    let input = "([])".to_string();
    let expected = true;
    assert_eq!(Solution::is_valid(input), expected)
}

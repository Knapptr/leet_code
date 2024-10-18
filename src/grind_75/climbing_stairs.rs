use crate::solution::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memoized = HashMap::from([(-1, 0), (0, 0), (1, 1), (2, 2)]);
        fn recurse(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            // check if memoized already
            if n <= 0 {
                return 0;
            }
            if let Some(value) = memo.get(&n) {
                return *value;
            } else {
                let sum = recurse(n - 1, memo) + recurse(n - 2, memo);
                memo.insert(n, sum);
                return sum;
            }
        }
        recurse(n, &mut memoized)
    }
}

#[cfg(test)]
#[test]
fn base_case() {
    let n = 3;
    let expected = 3;
    assert_eq!(Solution::climb_stairs(n), expected);
}
#[test]
fn large_case() {
    let n = 45;
    let expected = 1836311903;
    assert_eq!(Solution::climb_stairs(n), expected);
}

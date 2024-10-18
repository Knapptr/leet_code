use crate::solution::Solution;

/* returns (letter to go in place, whether to carry) */
fn add_bin_chars(l: char, r: char) -> (char, bool) {
    match (l, r) {
        ('1', '0') | ('0', '1') => ('1', false),
        ('1', '1') => ('0', true),
        ('0', '0') => ('0', false),
        _ => unreachable!(),
    }
}
impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let max_len = a.chars().count().max(b.chars().count());
        let mut return_string = String::new();
        // format strings accordingly
        a = format!("{:0>max_len$}", a);
        b = format!("{:0>max_len$}", b);
        let mut carrying = false;
        for (left, right) in a.chars().rev().zip(b.chars().rev()) {
            let mut sum = left;
            if carrying {
                let (n_sum, c2) = add_bin_chars(left, '1');
                sum = n_sum;
                carrying = c2;
            }
            let (sum, c3) = add_bin_chars(sum, right);
            carrying = carrying || c3;
            return_string.push(sum);
        }
        if carrying {
            return_string.push('1')
        }
        return_string.chars().rev().collect()
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let s1 = "11".to_string();
    let s2 = "1".to_string();
    let expected = "100".to_string();
    assert_eq!(Solution::add_binary(s1, s2), expected);
}
#[test]
fn example_two() {
    let s1 = "1010".to_string();
    let s2 = "1011".to_string();
    let expected = "10101".to_string();
    assert_eq!(Solution::add_binary(s1, s2), expected);
}

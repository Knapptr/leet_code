//Write an algorithm to determine if a number n is happy.

// A happy number is a number defined by the following process:

// Starting with any positive integer, replace the number by the sum of the squares of its digits.
// Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
// Those numbers for which this process ends in 1 are happy.
// Return true if n is a happy number, and false if not.

use std::collections::HashSet;

fn is_happy(mut n: i32) -> bool {
    // track list of sums so far
    let mut sums = HashSet::new();
    loop {
        // loop sum_squares n
        // if repeat sum, false
        sums.insert(n);
        n = sum_square_digits(n);
        if n == 1 {
            return true;
        }
        if sums.contains(&n) {
            return false;
        }
    }
}
fn sum_square_digits(n: i32) -> i32 {
    // break number into digits
    // square each digit
    // track sum
    let n_string = n.to_string();
    let digits = n_string.chars();
    let mut sum = 0;
    for digit in digits {
        let value = digit.to_digit(10).expect("Something went wrong") as i32;
        sum += value * value;
    }
    sum
}

#[cfg(test)]
#[test]
fn digit_sum() {
    assert_eq!(sum_square_digits(2), 4);
    assert_eq!(sum_square_digits(10), 1);
    assert_eq!(sum_square_digits(12), 5);
}

#[test]
fn happy_true() {
    let n = 19;
    let expected = true;

    assert_eq!(is_happy(n), expected);
}

#[test]
fn happy_false() {
    let n = 2;
    let expected = false;

    assert_eq!(is_happy(n), expected);
}

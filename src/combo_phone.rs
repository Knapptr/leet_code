// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.

// A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
//

fn letter_options(char: char) -> Vec<char> {
    let DIGIT_LETTERS: Vec<Vec<char>> = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
        vec!['j', 'k', 'l'],
        vec!['m', 'n', 'o'],
        vec!['p', 'q', 'r', 's'],
        vec!['t', 'u', 'v'],
        vec!['w', 'x', 'y', 'z'],
    ];
    let digit = char.to_digit(10).expect("Could not parse digit") - 2;
    DIGIT_LETTERS[digit as usize].clone()
}

fn backtrack(digits: &[char], solution: String, solutions: &mut Vec<String>) {
    if digits.len() == 0 {
        solutions.push(solution);
        return;
    }
    let current_digit = digits[0];
    for char in letter_options(current_digit) {
        let mut sol = solution.clone();
        sol.push(char);
        backtrack(&digits[1..], sol, solutions);
    }
}
fn letter_combinations(digits: String) -> Vec<String> {
    let digits = digits.chars().collect::<Vec<char>>();
    let mut solution = String::new();
    let mut solutions = Vec::new();
    backtrack(&digits[0..], solution, &mut solutions);
    solutions
}

#[cfg(test)]
#[test]
fn case_one() {
    let digits = "23".to_string();
    let expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];

    assert_eq!(letter_combinations(digits), expected);
}

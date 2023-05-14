// Given the integers zero, one, low, and high, we can construct a string by starting with an empty string, and then at each step perform either of the following:

// Append the character '0' zero times.
// Append the character '1' one times.
// This can be performed any number of times.

// A good string is a string constructed by the above process having a length between low and high (inclusive).

// Return the number of different good strings that can be constructed satisfying these properties. Since the answer can be large, return it modulo 109 + 7.
//

/* Solution Notes:
 * This solution is a brute force backtracking algorithm, which works for small cases but cannot
 * handle larger sets.
 *
 * "Dynamic Programming" will help here, but I don't yet understand that paradigm.
 * */
fn choices(zero: i32, one: i32) -> Vec<String> {
    let mut vec = Vec::new();
    vec.push("0".repeat(zero as usize));
    vec.push("1".repeat(one as usize));
    vec
}
fn backtrack(sol: String, solutions: &mut Vec<String>, low: i32, high: i32, zero: i32, one: i32) {
    if sol.len() > high as usize {
        return;
    }
    if sol.len() >= low as usize && sol.len() <= high as usize {
        solutions.push(sol.clone());
    }
    for choice in choices(zero, one) {
        let mut new_sol = sol.clone();
        new_sol.push_str(&choice);
        backtrack(new_sol, solutions, low, high, zero, one);
    }
}
fn construct_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    let mut solutions = Vec::new();
    let mut sol = String::new();

    backtrack(sol, &mut solutions, low, high, zero, one);

    (solutions.len() % (10i32.pow(9u32) as usize + 7)) as i32
}

#[cfg(test)]
#[test]
fn case_one() {
    let low = 3;
    let high = 3;
    let zero = 1;
    let one = 1;

    let expected = 8;

    assert_eq!(construct_good_strings(low, high, zero, one), expected);
}

#[test]
fn case_two() {
    let low = 2;
    let high = 3;
    let zero = 1;
    let one = 2;

    let expected = 5;

    assert_eq!(construct_good_strings(low, high, zero, one), expected);
}

// #[test]
// fn case_three() {
//     let low = 100;
//     let high = 100;
//     let zero = 10;
//     let one = 1;

//     let expected = 8;

//     assert_eq!(construct_good_strings(low, high, zero, one), expected);
// }
//// Helper tests
#[test]
fn choice_vec() {
    let zero = 2;
    let one = 4;

    let expected = vec!["00".to_string(), "1111".to_string()];

    assert_eq!(choices(zero, one), expected);
}

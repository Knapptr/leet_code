// A binary watch has 4 LEDs on the top to represent the hours (0-11), and 6 LEDs on the bottom to represent the minutes (0-59). Each LED represents a zero or one, with the least significant bit on the right.
//
// EGs (* denotes a lit up LED)
//
// H 8|4|*2|1
// M 32|16|8|4|*2|1
// would represent 2:02
//
// H 8|4|*2|*1
// M 32|16|8|*4|*2|1
// would represent 3:06
//

// Given an integer turnedOn which represents the number of LEDs that are currently on (ignoring the PM), return all possible times the watch could represent. You may return the answer in any order.

// The hour must not contain a leading zero.

// For example, "01:00" is not valid. It should be "1:00".
// The minute must be consist of two digits and may contain a leading zero.

// For example, "10:2" is not valid. It should be "10:02".

use std::collections::HashSet;

const HOUR_VALUES: [i32; 5] = [16, 8, 4, 2, 1];
const MINUTE_VALUES: [i32; 6] = [32, 16, 8, 4, 2, 1];

fn to_time_string(hour: i32, minute: i32) -> String {
    let minutes = format!("{minute:02}");
    let hour = format!("{hour}");
    format!("{hour}:{minutes}")
}

fn backtrack(
    remaining: i32,
    min_sol: i32,
    h_sol: i32,
    solutions: &mut HashSet<String>,
    rem_mins: Vec<i32>,
    rem_hours: Vec<i32>,
) {
    if remaining == 0 {
        solutions.insert(to_time_string(h_sol, min_sol));
        return;
    };
    // at each step there is an option of it being an hour led, or a minute LED
    // minutes max at 59, hours max at 12
    // if the hours are maxed, it has to be a minute
    // if the minutes are maxed, it has to be an hour
    if min_sol < 60 {
        for (i, min_val) in rem_mins.iter().enumerate() {
            let mut remainder_mins = rem_mins.clone();
            remainder_mins = remainder_mins.into_iter().skip(i + 1).collect();
            let next_val = min_sol + min_val;
            if next_val < 60 {
                backtrack(
                    remaining - 1,
                    next_val,
                    h_sol,
                    solutions,
                    remainder_mins,
                    rem_hours.clone(),
                );
            }
        }
    }
    if h_sol < 13 {
        for (i, h_val) in rem_hours.iter().enumerate() {
            let mut remainder_hours = rem_hours.clone();
            remainder_hours = remainder_hours.into_iter().skip(i + 1).collect();
            let next_val = h_sol + h_val;
            if next_val < 12 {
                backtrack(
                    remaining - 1,
                    min_sol,
                    next_val,
                    solutions,
                    rem_mins.clone(),
                    remainder_hours,
                );
            }
        }
    }
}

fn binary_watch(turned_on: i32) -> Vec<String> {
    let mut solutions = HashSet::new();
    backtrack(
        turned_on,
        0,
        0,
        &mut solutions,
        MINUTE_VALUES.to_vec(),
        HOUR_VALUES.to_vec(),
    );
    solutions.into_iter().collect()
}

#[cfg(test)]
#[test]
fn case_one() {
    let turned_on = 1;
    let mut expected = vec![
        "0:01".to_string(),
        "0:02".to_string(),
        "0:04".to_string(),
        "0:08".to_string(),
        "0:16".to_string(),
        "0:32".to_string(),
        "1:00".to_string(),
        "2:00".to_string(),
        "4:00".to_string(),
        "8:00".to_string(),
    ];
    expected.sort();
    let mut result = binary_watch(turned_on);
    result.sort();
    assert_eq!(result, expected);
}

#[test]
fn case_two() {
    let turned_on = 9;
<<<<<<< HEAD

    assert_eq!(binary_watch(turned_on).len(), 0);
=======

    assert_eq!(binary_watch(turned_on).len(), 0);
}

#[test]
fn case_three() {
    let turned_on = 2;

    let mut expected = vec![
        "0:03".to_string(),
        "0:05".to_string(),
        "0:06".to_string(),
        "0:09".to_string(),
        "0:10".to_string(),
        "0:12".to_string(),
        "0:17".to_string(),
        "0:18".to_string(),
        "0:20".to_string(),
        "0:24".to_string(),
        "0:33".to_string(),
        "0:34".to_string(),
        "0:36".to_string(),
        "0:40".to_string(),
        "0:48".to_string(),
        "1:01".to_string(),
        "1:02".to_string(),
        "1:04".to_string(),
        "1:08".to_string(),
        "1:16".to_string(),
        "1:32".to_string(),
        "2:01".to_string(),
        "2:02".to_string(),
        "2:04".to_string(),
        "2:08".to_string(),
        "2:16".to_string(),
        "2:32".to_string(),
        "3:00".to_string(),
        "4:01".to_string(),
        "4:02".to_string(),
        "4:04".to_string(),
        "4:08".to_string(),
        "4:16".to_string(),
        "4:32".to_string(),
        "5:00".to_string(),
        "6:00".to_string(),
        "8:01".to_string(),
        "8:02".to_string(),
        "8:04".to_string(),
        "8:08".to_string(),
        "8:16".to_string(),
        "8:32".to_string(),
        "9:00".to_string(),
        "10:00".to_string(),
    ];

    expected.sort();
    let mut result = binary_watch(turned_on);
    result.sort();
    println!("EXPECT: {expected:#?}");
    println!("RESULT: {result:#?}");
    println!("{}", result.len() - expected.len());

    assert_eq!(result, expected);
>>>>>>> c13c95a (Adds solutions)
}

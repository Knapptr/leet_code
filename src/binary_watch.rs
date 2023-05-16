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
    solutions: &mut Vec<String>,
    rem_mins: Vec<i32>,
    rem_hours: Vec<i32>,
) {
    if remaining == 0 {
        solutions.push(to_time_string(h_sol, min_sol));
        return;
    };
    // at each step there is an option of it being an hour led, or a minute LED
    // minutes max at 59, hours max at 12
    // if the hours are maxed, it has to be a minute
    // if the minutes are maxed, it has to be an hour
    if min_sol < 60 {
        for (i, min_val) in rem_mins.iter().enumerate() {
            let mut remainder_mins = rem_mins.clone();
            remainder_mins.remove(i);
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
            remainder_hours.remove(i);
            let next_val = h_sol + h_val;
            if next_val < 13 {
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
    let mut solutions = Vec::new();
    backtrack(
        turned_on,
        0,
        0,
        &mut solutions,
        MINUTE_VALUES.to_vec(),
        HOUR_VALUES.to_vec(),
    );
    solutions
}

#[cfg(test)]
#[test]
fn case_one() {
    let turned_on = 1;
    let expected = vec![
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

    assert_eq!(binary_watch(turned_on).len(), expected.len());
}
#[test]
fn case_two() {
    let turned_on = 9;

    assert_eq!(binary_watch(turned_on).len(), 0);
}

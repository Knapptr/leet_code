pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    // 2 pointers
    let chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len();
    let mut distances = vec![i32::MAX; chars.len()];

    let mut last_occurance_left = None;
    let mut last_occurance_right = None;

    while left < chars.len() && right >= 0 {
        right -= 1;
        if chars[left] == c {
            last_occurance_left = Some(left)
        }
        if chars[right] == c {
            last_occurance_right = Some(right)
        }
        match (last_occurance_left, last_occurance_right) {
            (None, None) => (),
            (Some(l), Some(r)) => {
                distances[left] = (left as i32 - l as i32).abs().min(distances[left]);
                distances[right] = (right as i32 - r as i32).abs().min(distances[right]);
            }
            (Some(l), None) => {
                distances[left] = (left as i32 - l as i32).abs().min(distances[left]);
                distances[right] = (right as i32 - l as i32).abs();
            }
            (None, Some(r)) => {
                distances[right] = (right as i32 - r as i32).abs().min(distances[right]);
                distances[left] = (left as i32 - r as i32).abs();
            }
        }
        left += 1;
    }
    distances

    // 2 Loops
    // let chars = s.chars();
    // let mut distances = vec![];
    // // get all occurances of char
    // let mut occurances_of_char = vec![];
    // for (index, char) in chars.enumerate() {
    //     if char == c {
    //         occurances_of_char.push(index)
    //     };
    // }
    // // iterate through characters and find shortest distance to occurance
    // let chars = s.chars();
    // for (index, char) in chars.enumerate() {
    //     // min difference (abs) between index and occurance
    //     let mut min_distance = i32::MAX;
    //     for occurance in &occurances_of_char {
    //         min_distance = (index as i32 - *occurance as i32)
    //             .abs()
    //             .min(min_distance as i32)
    //     }
    //     distances.push(min_distance)
    // }
    // distances
}

#[cfg(test)]
#[test]
fn simple() {
    let input = "aaab".to_string();
    let target = 'b';
    assert_eq!(shortest_to_char(input, target), vec![3, 2, 1, 0]);
}

#[test]
fn leet_code() {
    let input = "loveleetcode".to_string();
    let target = 'e';
    let expected = vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0];

    assert_eq!(shortest_to_char(input, target), expected);
}

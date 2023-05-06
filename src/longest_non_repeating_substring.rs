//Given a string s, find the length of the longest substring without repeating characters.

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    };

    let mut last_seen = std::collections::HashMap::new();

    let chars: Vec<char> = s.chars().collect();

    let mut start = 0;
    let mut end = 1;
    let mut longest = end - start;

    last_seen.insert(chars[start], start);
    while end < chars.len() {
        if let Some(index) = last_seen.get_mut(&chars[end]) {
            if *index >= start {
                start += 1;
                *index = end;
            }
            end += 1;
        } else {
            *last_seen.entry(chars[end]).or_insert(end) = end;
            longest = longest.max(end - start + 1);
            end += 1;
        }
    }
    longest as i32
}

#[cfg(test)]
#[test]
fn zero() {
    let s = "".to_string();
    let expected = 0;

    assert_eq!(length_of_longest_substring(s), expected);
}

#[test]
fn single() {
    let s = "bb".to_string();
    let expected = 1;

    assert_eq!(length_of_longest_substring(s), expected);
}

#[test]
fn abcabcbb() {
    let s = "abcabcbb".to_string();
    let expected = 3;

    assert_eq!(length_of_longest_substring(s), expected);
}

#[test]
fn dvdf() {
    let s = "dvdf".to_string();
    let expected = 3;

    assert_eq!(length_of_longest_substring(s), expected);
}

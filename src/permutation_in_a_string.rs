//Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.In other words, return true if one of s1's permutations is the substring of s2.

fn check_inclusion(s1: String, s2: String) -> bool {
    if s2.len() < s1.len() {
        return false;
    }
    let mut sub_count = [0; 26];

    for char in s1.chars() {
        sub_count[char as usize - b'a' as usize] += 1;
    }

    let mut map = [0; 26];

    // create first HashMap
    for char in s2.chars().take(s1.len()) {
        map[char as usize - b'a' as usize] += 1;
        if map == sub_count {
            return true;
        };
    }
    // slide window
    for i in s1.len()..s2.len() {
        //remove previous
        map[s2.chars().nth(i - s1.len()).unwrap() as usize - s1.len() as usize - b'a' as usize] -=
            1;
        // add next
        map[s2.chars().nth(i).unwrap() as usize - s1.len() as usize - b'a' as usize] += 1;
        if map == sub_count {
            return true;
        };
    }
    false
}

#[cfg(test)]
#[test]
fn simple_tests() {
    assert!(check_inclusion("adc".to_string(), "dcda".to_string()));
    assert!(check_inclusion("ab".to_string(), "eidbaooo".to_string()));
    assert!(!check_inclusion("ab".to_string(), "eidboaoo".to_string()));
    assert!(!check_inclusion(
        "hello".to_string(),
        "ooolleoooleh".to_string()
    ));
}

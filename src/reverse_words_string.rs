pub fn reverse_words(s: String) -> String {
    let mut rev_string = String::new();
    s.split_whitespace()
        .map(|w| w.chars().rev().collect::<String>())
        .for_each(|w| rev_string.push_str(&format!(" {}", w)));
    rev_string.trim().to_string()
}

#[cfg(test)]
#[test]
fn single_word() {
    let input = "Hello".to_string();
    let expected = "olleH".to_string();

    assert_eq!(reverse_words(input), expected);
}

#[test]
fn two_words() {
    let input = "Hello World".to_string();
    let expected = "olleH dlroW".to_string();

    assert_eq!(reverse_words(input), expected);
}

// You are given an array items, where each items[i] = [typei, colori, namei] describes the type, color, and name of the ith item. You are also given a rule represented by two strings, ruleKey and ruleValue.

// The ith item is said to match the rule if one of the following is true:

// ruleKey == "type" and ruleValue == typei.
// ruleKey == "color" and ruleValue == colori.
// ruleKey == "name" and ruleValue == namei.
// Return the number of items that match the given rule.
//

pub struct Solution;

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut rule_index: i32 = -1;
        match rule_key.as_ref() {
            "color" => rule_index = 1,
            "type" => rule_index = 0,
            "name" => rule_index = 2,
            _ => rule_index = unreachable!(),
        }

        let mut count = 0;

        for item in items {
            if item[rule_index as usize] == rule_value {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let items = vec![
        vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
        vec![
            "computer".to_string(),
            "silver".to_string(),
            "lenovo".to_string(),
        ],
        vec![
            "phone".to_string(),
            "gold".to_string(),
            "iphone".to_string(),
        ],
    ];
    let rule_key = "color".to_string();
    let rule_value = "silver".to_string();

    let expected_count = 1;
    let result = Solution::count_matches(items, rule_key, rule_value);

    assert_eq!(expected_count, result);
}

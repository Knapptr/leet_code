use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (idx, num) in nums.iter().enumerate() {
        let target_diff = target - num;
        if let Some(idx_t) = map.get(&target_diff) {
            return vec![*idx_t, idx as i32];
        } else {
            map.insert(num, idx as i32);
        }
    }
    vec![]
}

#[cfg(test)]
#[test]
fn three_values() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let expected = vec![1, 2];

    assert_eq!(two_sum(nums, target), expected);
}

#[test]
fn four_values() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let expected = vec![0, 1];

    assert_eq!(two_sum(nums, target), expected);
}

#[test]
fn doubles() {
    let nums = vec![3,3];
    let target = 6;
    let expected = vec![0, 1];

    assert_eq!(two_sum(nums, target), expected);
}



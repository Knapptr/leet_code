use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = (nums.len() - 1) as i32;

    while low <= high {
        let midpoint = low + (high - low) / 2;
        let current = nums[midpoint as usize];
        match current.cmp(&target) {
            Ordering::Less => low = midpoint + 1,
            Ordering::Greater => high = midpoint - 1,
            Ordering::Equal => return midpoint as i32,
        }
    }
    return -1;
}

#[cfg(test)]
#[test]
fn exists() {
    let input_nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;

    assert_eq!(search(input_nums, target), 4);
}

#[test]
fn not_exist() {
    let input_nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 2;

    assert_eq!(search(input_nums, target), -1);
}
#[test]
fn single_element() {
    let input_nums = vec![5];
    let target = -5;

    assert_eq!(search(input_nums, target), -1);
}

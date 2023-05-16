// Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:

// 0 <= a, b, c, d < n
// a, b, c, and d are distinct.
// nums[a] + nums[b] + nums[c] + nums[d] == target
// You may return the answer in any order.
//
use std::collections::{HashMap, HashSet};

// recursively make map of pair values using backtracking
fn map_pairs(
    nums: Vec<i32>,
    mut set: Vec<(i32, usize)>,
    pairs: &mut HashMap<i32, Vec<Vec<(i32, usize)>>>,
    index: usize,
) {
    if set.len() == 2 {
        let sum = set[0].0 + set[1].0;
        let pair_vec = pairs.entry(sum).or_insert(Vec::new());
        pair_vec.push(set.clone());
        // set.pop();
        return;
    }
    if index == nums.len() {
        return;
    }
    for (i, num) in nums.iter().enumerate().skip(index) {
        set.push((*num, i));
        map_pairs(nums.clone(), set.clone(), pairs, i + 1);
        set.pop();
    }
}

fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut pairs = HashMap::new();
    map_pairs(nums, Vec::new(), &mut pairs, 0);

    // Allocate solution space
    let mut sols_set = HashSet::new();
    let mut seen = HashSet::new();
    // iterate through pair sums, and do 'twosum'.
    for (sum, cur_pairs) in &pairs {
        let remaining = target - sum;
        if seen.contains(sum) || seen.contains(&remaining) {
            continue;
        }
        seen.insert(*sum);
        seen.insert(remaining);
        if pairs.contains_key(&remaining) {
            let matching_pairs = pairs.get(&remaining).expect("Something went wrong");
            // compare each pair of cur_pairs with matching pairs
            for (ci, c_pair) in cur_pairs.iter().enumerate() {
                let cx = c_pair[0].0;
                let cxi = c_pair[0].1;
                let cy = c_pair[1].0;
                let cyi = c_pair[1].1;
                for mi in ci..matching_pairs.len() {
                    let pair = &matching_pairs[mi];
                    let mx = pair[0].0;
                    let mxi = pair[0].1;
                    let my = pair[1].0;
                    let myi = pair[1].1;

                    //compare cxi to mxi, myi
                    //compare cyi to mxi, myi
                    if (cxi != mxi && cxi != myi) && (cyi != mxi && cyi != myi) {
                        let mut sol = vec![mx, my, cx, cy];
                        sol.sort();
                        sols_set.insert(sol);
                    }
                }
            }
        }
    }

    sols_set.into_iter().collect()
}

#[cfg(test)]
#[test]
fn pairs() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let mut pairs = HashMap::new();
    map_pairs(nums, Vec::new(), &mut pairs, 0);
    let mut all_pairs: Vec<_> = pairs.into_values().collect();
    let count = all_pairs.iter().fold(0, |acc, x| acc + x.len());

    assert_eq!(count, 15);
}
#[test]
fn example_1() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let mut expected = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
    let mut result = four_sum(nums, target);
    expected.sort();
    result.sort();

    assert_eq!(result, expected);
}

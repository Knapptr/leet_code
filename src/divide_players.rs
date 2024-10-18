use std::collections::HashMap;

use crate::solution::Solution;

// You are given a positive integer array skill of even length n where skill[i] denotes the skill of the ith player. Divide the players into n / 2 teams of size 2 such that the total skill of each team is equal.

// The chemistry of a team is equal to the product of the skills of the players on that team.

// Return the sum of the chemistry of all the teams, or return -1 if there is no way to divide the players into teams such that the total skill of each team is equal.

impl Solution {
    /* O(n log n)
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        // sort the players.
        skill.sort();
        // initial sum
        let first = *skill.first().unwrap();
        let last = *skill.last().unwrap();
        let target_sum = first + last;
        // sum all others
        let mut left = 1usize;
        let mut right = skill.len() - 2;
        let mut teams = vec![(first, last)];
        while left < right {
            //check the sum
            if skill[left] + skill[right] != target_sum {
                return -1;
            }
            teams.push((skill[left], skill[right]));
            left += 1;
            right -= 1;
        }
        println!("Teams: {teams:?}");
        return teams.into_iter().map(|t| (t.0 * t.1) as i64).sum();
    }
    */
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        // count frequencies
        // calc min and max
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut freq = HashMap::new();
        for score in &skill {
            min = min.min(*score);
            max = max.max(*score);
            let entry = freq.entry(score).or_insert(0);
            *entry += 1;
        }
        // find sum of min max
        let target_sum = min + max;
        // iterate through skills and check if complement is there
        let mut sum = 0;
        for score in &skill {
            match freq.get_mut(score) {
                None => unreachable!(),
                Some(current_count) => {
                    // Skip the current number because it has already been teamed up
                    if *current_count == 0 {
                        continue;
                    }
                    *current_count -= 1
                }
            }
            match freq.get_mut(&(target_sum - score)) {
                None => return -1,
                Some(count) => {
                    // if it isn't (or value is 0) return -1
                    if *count == 0 {
                        return -1;
                    }
                    // if it is, decrement by one
                    *count -= 1;

                    sum += score * (target_sum - score);
                }
            }
        }
        return sum as i64;
        // Teams have been made
    }
}
#[cfg(test)]
#[test]
fn example_1() {
    // use crate::solution::Solution;

    let skill = vec![3, 2, 5, 1, 3, 4];
    let expected = 22i64;
    assert_eq!(Solution::divide_players(skill), expected);
}
#[test]
fn example_2() {
    // use crate::solution::Solution;

    let skill = vec![3, 4];
    let expected = 12i64;
    assert_eq!(Solution::divide_players(skill), expected);
}

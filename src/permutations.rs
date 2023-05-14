// Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.

use std::collections::{HashSet, VecDeque};

fn backtrack(
    nums: Vec<i32>,
    mut solution: Vec<i32>,
    solutions: &mut Vec<Vec<i32>>,
) -> &mut Vec<Vec<i32>> {
    if nums.len() == 0 {
        solutions.push(solution.clone());
    }

    for i in 0..nums.len() {
        let mut curr_nums = nums.clone();
        let current_num = curr_nums.remove(i);
        let mut curr_sol = solution.clone();
        curr_sol.push(current_num);
        backtrack(curr_nums, curr_sol, solutions);
    }
    solutions
}
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut sol = Vec::new();
    let mut sols = Vec::new();
    backtrack(nums, sol, &mut sols);
    sols
}

fn iterative(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut stack = Vec::new();
    let mut solutions = Vec::new();

    for num in &nums {
        let mut used = HashSet::new();
        let mut solution = Vec::new();
        used.insert(*num);
        solution.push(*num);
        stack.push((solution, used));
    }

    while let Some((sol, set)) = stack.pop() {
        if sol.len() == nums.len() {
            solutions.push(sol);
            continue;
        }
        for num in &nums {
            if !set.contains(num) {
                let mut new_set = set.clone();
                let mut new_sol = sol.clone();
                new_sol.push(*num);
                new_set.insert(*num);
                stack.push((new_sol, new_set));
            }
        }
    }
    solutions.reverse();
    solutions
}

#[cfg(test)]
#[test]
fn simple() {
    let nums = vec![1, 2, 3];
    let expected = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];

    assert_eq!(permute(nums), expected);
}

#[test]
fn simple_iterative() {
    let nums = vec![1, 2, 3];
    let expected = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];

    assert_eq!(iterative(nums), expected);
}

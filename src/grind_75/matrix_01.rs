use std::collections::{HashSet, VecDeque};

use crate::solution::Solution;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Using BFS
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        // get all 0s
        for (iy, row) in mat.iter().enumerate() {
            for (ix, num) in row.iter().enumerate() {
                if *num == 0 {
                    queue.push_back(((iy as i32, ix as i32), 0));
                    seen.insert((iy as i32, ix as i32));
                }
            }
        }
        while let Some((coords, depth)) = queue.pop_front() {
            mat[coords.0 as usize][coords.1 as usize] = depth;
            // add unseen neighbors to queue
            for (delta_y, delta_x) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_coords = (coords.0 + delta_y, coords.1 + delta_x);
                if !seen.contains(&new_coords) {
                    if new_coords.0 >= 0
                        && new_coords.0 < mat.len() as i32
                        && new_coords.1 >= 0
                        && new_coords.1 < mat[0].len() as i32
                    {
                        queue.push_back((new_coords, depth + 1));
                        seen.insert(new_coords);
                    }
                }
            }
        }
        mat
        /*

        // my intuition is to solve this with DP
        let mut dp_table = vec![vec![i32::MAX - 1000; mat[0].len()]; mat.len()];
        fn iterate(mat: &Vec<Vec<i32>>, dp_table: &mut Vec<Vec<i32>>) {
            for (iy, row) in mat.iter().enumerate().rev() {
                for (ix, num) in row.iter().enumerate().rev() {
                    if *num == 0 {
                        dp_table[iy][ix] = 0;
                        if let Some(mut x) = dp_table[iy].get_mut(ix + 1) {
                            *x = *x.min(&mut 1);
                        }
                        if ix > 0 {
                            let x = dp_table.get_mut(iy).unwrap().get_mut(ix - 1).unwrap();
                            *x = *x.min(&mut 1);
                        }
                        if let Some(y) = dp_table.get_mut(iy + 1) {
                            if let Some(mut x) = y.get_mut(ix) {
                                *x = *x.min(&mut 1);
                            }
                            if let Some(mut x) = y.get_mut(ix) {
                                *x = *x.min(&mut 1);
                            }
                        }
                        if iy > 0 {
                            if let Some(y) = dp_table.get_mut(iy - 1) {
                                if let Some(x) = y.get_mut(ix) {
                                    *x = *x.min(&mut 1);
                                }
                                if let Some(x) = y.get_mut(ix) {
                                    *x = *x.min(&mut 1);
                                }
                            }
                        }
                    } else {
                        dp_table[iy][ix] = 1 + std::cmp::min(
                            std::cmp::min(
                                dp_table
                                    .get(iy + 1)
                                    .unwrap_or(&vec![])
                                    .get(ix)
                                    .unwrap_or(&i32::MAX),
                                if iy > 0 {
                                    dp_table[iy - 1].get(ix).unwrap_or(&(i32::MAX - 1))
                                } else {
                                    &(i32::MAX)
                                },
                            ),
                            std::cmp::min(
                                dp_table
                                    .get(iy)
                                    .unwrap_or(&vec![])
                                    .get(ix + 1)
                                    .unwrap_or(&i32::MAX),
                                if ix > 0 {
                                    dp_table[iy].get(ix - 1).unwrap_or(&(i32::MAX - 1))
                                } else {
                                    &(i32::MAX)
                                },
                            ),
                        )
                    }
                }
                // println!("{dp_table:?}");
            }
            for (iy, row) in mat.iter().enumerate() {
                for (ix, num) in row.iter().enumerate() {
                    if *num == 0 {
                        dp_table[iy][ix] = 0;
                        if let Some(mut x) = dp_table[iy].get_mut(ix + 1) {
                            *x = *x.min(&mut 1);
                        }
                        if ix > 0 {
                            let x = dp_table.get_mut(iy).unwrap().get_mut(ix - 1).unwrap();
                            *x = *x.min(&mut 1);
                        }
                        if let Some(y) = dp_table.get_mut(iy + 1) {
                            if let Some(mut x) = y.get_mut(ix) {
                                *x = *x.min(&mut 1);
                            }
                            if let Some(mut x) = y.get_mut(ix) {
                                *x = *x.min(&mut 1);
                            }
                        }
                        if iy > 0 {
                            if let Some(y) = dp_table.get_mut(iy - 1) {
                                if let Some(x) = y.get_mut(ix) {
                                    *x = *x.min(&mut 1);
                                }
                                if let Some(x) = y.get_mut(ix) {
                                    *x = *x.min(&mut 1);
                                }
                            }
                        }
                    } else {
                        dp_table[iy][ix] = 1 + std::cmp::min(
                            std::cmp::min(
                                dp_table
                                    .get(iy + 1)
                                    .unwrap_or(&vec![])
                                    .get(ix)
                                    .unwrap_or(&i32::MAX),
                                if iy > 0 {
                                    dp_table[iy - 1].get(ix).unwrap_or(&(i32::MAX - 1))
                                } else {
                                    &(i32::MAX)
                                },
                            ),
                            std::cmp::min(
                                dp_table
                                    .get(iy)
                                    .unwrap_or(&vec![])
                                    .get(ix + 1)
                                    .unwrap_or(&i32::MAX),
                                if ix > 0 {
                                    dp_table[iy].get(ix - 1).unwrap_or(&(i32::MAX - 1))
                                } else {
                                    &(i32::MAX)
                                },
                            ),
                        )
                    }
                }
                // println!("{dp_table:?}");
            }
        }

        iterate(&mat, &mut dp_table);
        dp_table
            */
    }
}
#[cfg(test)]
#[test]
fn example_one() {
    let matrix = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    assert_eq!(Solution::update_matrix(matrix), expected);
}
#[test]
fn example_two() {
    let matrix = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];
    assert_eq!(Solution::update_matrix(matrix), expected);
}

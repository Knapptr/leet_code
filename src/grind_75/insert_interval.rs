use crate::solution::Solution;
#[derive(Debug)]
enum MergeCase {
    Before(usize),
    Merge(usize),
    After(usize),
}
impl Solution {
    fn insert_interval(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![new_interval];
        };
        let mut index = 0;
        let mut merged = Vec::new();

        while index < intervals.len() {
            let current_element = &intervals[index];
            if current_element[1] < new_interval[0] {
                merged.push(current_element.clone());
                index += 1;
            } else {
                break;
            }
        }

        let mut to_push = vec![
            new_interval[0].min(intervals.get(index).unwrap_or(&vec![i32::MAX])[0]),
            new_interval[1],
        ];
        while index < intervals.len() {
            let mut current_element = &intervals[index];
            if to_push[1] >= current_element[0] {
                to_push[1] = to_push[1].max(current_element[1]);
                index += 1
            } else {
                break;
            }
        }
        merged.push(to_push);
        while index < intervals.len() {
            merged.push(intervals[index].clone());
            index += 1;
        }
        merged
        /* My Orignal, working but extremely long solution -------------------------------------
        /*
        Find the index of the interval that the start point of the new interval fits inside (binary search)
                -- What if the start point is between a point?
                -- What if the start point is after all?
                -- What if the start point is before all?
                        */
        // Find the start index
        let mut current_slice = &intervals[0..];
        let mut start_index = 0i32;
        loop {
            if current_slice.len() == 0 {
                break;
            }
            let mid_point_index = current_slice.len() / 2;
            let mid_point_start = current_slice[mid_point_index][0];
            let mid_point_end = current_slice[mid_point_index][1];

            println!("Intervals: {intervals:?}\nNew Interval: {new_interval:?}\nCurrent Slice:{current_slice:?}\nmid-index:{mid_point_index}\n--\n");

            if new_interval[0] > mid_point_end {
                current_slice = &current_slice[mid_point_index + 1..];
                start_index += (mid_point_index + 1) as i32;
            }

            if new_interval[0] < mid_point_start {
                current_slice = &current_slice[0..mid_point_index];
                if current_slice.len() == 0 {
                    start_index -= 1;
                    break;
                }
            }

            if new_interval[0] >= mid_point_start && new_interval[0] <= mid_point_end {
                start_index += (mid_point_index) as i32;
                break;
            }
        }
        println!("Start Index: {start_index:?}");

        // Handle Append Case
        if start_index >= intervals.len() as i32 {
            intervals.push(new_interval);
            // return intervals;
            return intervals;
        }

        // Find End Index
        // Needs to handle Prepend Case
        let mut end_index = start_index.max(0);
        current_slice = &intervals[end_index as usize..];
        loop {
            if current_slice.len() == 0 {
                break;
            }
            let mid_point_index = current_slice.len() / 2;
            let mid_point_start = current_slice[mid_point_index][0];
            let mid_point_end = current_slice[mid_point_index][1];

            if new_interval[1] < mid_point_start {
                current_slice = &current_slice[0..mid_point_index];
                // handle case of that being last slice (prepend) ??
                // if current_slice.len() == 0 {
                //     end_index -= 1;
                //     break;
                // }
            }

            if new_interval[1] > mid_point_end {
                current_slice = &current_slice[mid_point_index + 1..];
                // if current_slice.len() != 0 {
                end_index += (mid_point_index + 1) as i32
                // }
            }

            if new_interval[1] <= mid_point_end && new_interval[1] >= mid_point_start {
                end_index += mid_point_index as i32;
                break;
            }
        }

        // The end_index could be inclusive or exclusive, so test for that
        if end_index < intervals.len() as i32 {
            if new_interval[1] < intervals[end_index as usize][0] {
                end_index -= 1
            }
            // if new_interval[1] > intervals[end_index][0]{end_index -= 1}
        }

        // Handle prepend
        if end_index == -1 {
            let mut new_vec = vec![new_interval];
            new_vec.append(&mut intervals);
            return new_vec;
        }

        // handle no no_conflict_inserton
        if end_index < start_index {
            // Before insertion
            let mut new_vec = intervals[0..start_index as usize].to_vec();
            // Insert Element
            new_vec.push(new_interval);
            // After Insertion
            new_vec.append(&mut intervals[start_index as usize..].to_vec());
            return new_vec;
        }

        // Handle Merging
        // Check if start index interval includes the current start
        if start_index > 0 {
            if new_interval[0] > intervals[start_index as usize][1] {
                start_index += 1;
            }
        }
        let inserted_interval = vec![
            new_interval[0].min(
                intervals
                    .get(start_index as usize)
                    .unwrap_or(&vec![i32::MAX, 0])[0],
            ),
            new_interval[1].max(
                intervals
                    .get(end_index as usize)
                    .unwrap_or(&vec![0, i32::MIN])[1],
            ),
        ];
        let mut pre_values: Vec<Vec<i32>> = intervals
            .iter()
            .take(start_index.max(0) as usize)
            .map(|i| i.to_vec())
            .collect();
        pre_values.push(inserted_interval);

        let mut post_values: Vec<Vec<i32>> =
            intervals.into_iter().skip(end_index as usize + 1).collect();
        pre_values.append(&mut post_values);
        println!("End Index: {end_index}");
        return pre_values;

        // (start_index, end_index)
        // todo!()
        */
    }
}
// [[1,3],[6,9]] + [2,5] -> [[1,5],[6,9]]
// [[1,2],[3,5],[6,7]] + [2,8] = [[1,8]]
#[cfg(test)]
#[test]
fn no_conflict_inserton() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![4, 5];
    let expected = vec![vec![1, 3], vec![4, 5], vec![6, 9]];
    // let expected = (1, 0);
    assert_eq!(Solution::insert_interval(intervals, new_interval), expected);
}
#[test]
fn example_one() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    let expected = vec![vec![1, 5], vec![6, 9]];
    // let expected = (0, 0);
    assert_eq!(Solution::insert_interval(intervals, new_interval), expected);
}
#[test]
fn example_two() {
    let intervals = vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ];
    let new_interval = vec![4, 8];
    let expected = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
    // let expected = (1, 3);
    assert_eq!(Solution::insert_interval(intervals, new_interval), expected);
}
#[test]
fn empty_set() {
    let intervals = vec![];
    let new_interval = vec![2, 5];
    let expected = vec![vec![2, 5]];
    // let expected = (0, 0);
    assert_eq!(Solution::insert_interval(intervals, new_interval), expected);
}
#[test]
fn append_case() {
    let intervals = vec![vec![1, 2], vec![4, 5], vec![6, 7]];
    let new_interval = vec![9, 10];
    let expected = vec![vec![1, 2], vec![4, 5], vec![6, 7], vec![9, 10]];
    // let expected = (3, 3);
    assert_eq!(Solution::insert_interval(intervals, new_interval), expected);
}
#[test]
fn case_failed() {
    let intervals = vec![vec![1, 5]];
    let new_interval = vec![5, 7];
    let expected = vec![vec![1, 7]];
    // let expected = (0, 1);
    assert_eq!(Solution::insert_interval(intervals, new_interval), expected);
}
#[test]
fn with_merge() {
    let intervals = vec![vec![0, 3], vec![4, 6], vec![8, 10]];
    let new_interval = vec![7, 13];
    let expected = vec![vec![0, 3], vec![4, 6], vec![7, 13]];
    assert_eq!(Solution::insert_interval(intervals, new_interval), expected);
}
#[test]
fn prepend_merge() {
    let intervals = vec![vec![1, 5]];
    let new_interval = vec![0, 3];
    let expected = vec![vec![0, 5]];
    // let expected = (0, 1);
    assert_eq!(Solution::insert_interval(intervals, new_interval), expected);
}
#[test]
fn failed_leet_2() {
    let intervals = vec![vec![2, 5], vec![6, 7], vec![8, 9]];
    let new_interval = vec![0, 10];
    let expected = vec![vec![0, 10]];
    // let expected = (-1, 3);
    assert_eq!(Solution::insert_interval(intervals, new_interval), expected);
}
#[test]
fn prepend_case() {
    let intervals = vec![vec![2, 5]];
    let new_interval = vec![0, 1];
    let expected = vec![vec![0, 1], vec![2, 5]];
    // let expected = (-1, -1);
    assert_eq!(Solution::insert_interval(intervals, new_interval), expected);
}

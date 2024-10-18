use std::collections::VecDeque;

use crate::solution::Solution;

/*You are given an image represented by an m x n grid of integers image, where image[i][j] represents the pixel value of the image. You are also given three integers sr, sc, and color. Your task is to perform a flood fill on the image starting from the pixel image[sr][sc].

To perform a flood fill:

    Begin with the starting pixel and change its color to color.
    Perform the same process for each pixel that is directly adjacent (pixels that share a side with the original pixel, either horizontally or vertically) and shares the same color as the starting pixel.
    Keep repeating this process by checking neighboring pixels of the updated pixels and modifying their color if it matches the original color of the starting pixel.
    The process stops when there are no more adjacent pixels of the original color to update.

Return the modified image after performing the flood fill.*/

impl Solution {
    fn flood_fill_recursive(
        mut image: Vec<Vec<i32>>,
        sr: i32,
        sc: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        fn recurse(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, target: i32) {
            let original_color = image[sr as usize][sc as usize];
            // base case - The target is the original color
            if original_color == target {
                return;
            }
            //change color
            image[sr as usize][sc as usize] = target;
            // recurse on valid neighbors
            for (mod_x, mod_y) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (test_x, test_y) = (mod_x + sr, mod_y + sc);
                if test_x < 0 {
                    continue;
                }
                if test_y < 0 {
                    continue;
                }
                if test_x as usize >= image.len() {
                    continue;
                }
                if test_y as usize >= image[0].len() {
                    continue;
                }
                if image[test_x as usize][test_y as usize] == original_color {
                    recurse(image, test_x, test_y, target)
                };
            }
        }
        recurse(&mut image, sr, sc, target);
        return image;
    }
    fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, target: i32) -> Vec<Vec<i32>> {
        let (x, y) = (sr as usize, sc as usize);
        let (height, len) = (image.len(), image[0].len());
        let neighbor_sets: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        //bfs
        let mut queue = VecDeque::new();
        if image[x][y] != target {
            queue.push_back((x, y))
        }
        let original_color = image[x][y];
        while queue.len() > 0 {
            let (current_x, current_y) = queue.pop_front().unwrap();
            // get neighbors
            for (mod_x, mod_y) in &neighbor_sets {
                let (test_x, test_y) = (*mod_x + current_x as i32, *mod_y + current_y as i32);
                if test_x as i32 >= len as i32 || test_x < 0 {
                    continue;
                }
                if test_y >= height as i32 || test_y < 0 {
                    continue;
                }
                // if neighbor is original color, add it to the queue
                if image[test_x as usize][test_y as usize] == original_color {
                    queue.push_back((test_x as usize, test_y as usize));
                }
            }
            // change current color
            image[current_x][current_y] = target;
        }
        image
    }
}
#[cfg(test)]
#[test]
fn example_one() {
    let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
    assert_eq!(Solution::flood_fill(image, 1, 1, 2), expected);
}
#[test]
fn example_one_recursive() {
    let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
    assert_eq!(Solution::flood_fill_recursive(image, 1, 1, 2), expected);
}

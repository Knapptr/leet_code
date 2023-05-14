use std::collections::{HashSet, VecDeque};
fn bfs(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::*;
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut queue = VecDeque::new();
    let mut dist = vec![vec![i32::MAX; mat[0].len()]; mat.len()];

    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if mat[i][j] == 0 {
                dist[i][j] = 0;
                queue.push_back((i as i32, j as i32));
            }
        }
    }

    while let Some((i, j)) = queue.pop_front() {
        for d in DIR {
            let i1 = i + d.0;
            let j1 = j + d.1;
            if (i1 >= 0 && i1 < mat.len() as i32)
                && (j1 >= 0 && j1 < mat[0].len() as i32)
                && dist[i as usize][j as usize] + 1 < dist[i1 as usize][j1 as usize]
            {
                dist[i1 as usize][j1 as usize] = dist[i as usize][j as usize] + 1;
                queue.push_back((i1, j1));
            }
        }
    }
    dist
}

pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let y_size = mat.len();
    let x_size = mat.get(0).unwrap().len();
    let mut mat_map = vec![vec![i32::MAX; x_size]; y_size];
    let mut stack: VecDeque<(usize, usize, i32)> = VecDeque::new();
    let mut seen = HashSet::new();

    for y in 0..y_size {
        for x in 0..x_size {
            if mat[y][x] == 0 {
                stack.push_front((y, x, 0));
                mat_map[y][x] = 0;
                seen.insert((y, x));
            }
        }
    }
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some((y_coord, x_coord, dist)) = stack.pop_front() {
        for (y_delta, x_delta) in DIRECTIONS {
            let (current_y, current_x) = ((y_coord as i32 + y_delta), (x_coord as i32 + x_delta));
            if (current_y < y_size as i32)
                && (current_y >= 0)
                && (current_x < x_size as i32)
                && (current_x >= 0)
                && !seen.contains(&(current_y as usize, current_x as usize))
            {
                mat_map[current_y as usize][current_x as usize] =
                    mat_map[current_y as usize][current_x as usize].min(dist + 1);
                stack.push_back((current_y as usize, current_x as usize, dist + 1));
                seen.insert((y_coord as usize, x_coord as usize));
            }
        }
    }
    mat_map
}

// 0 based implementation
/*This works, but a suggested optimization is to use 0's instead of 1's,
 * as a 0 will always be a correct endpoint*/
//
// while let Some((y_coord, x_coord, dist)) = stack.pop_front() {
//     if mat[y_coord][x_coord] == 0 {
//         mat_map[y][x] = dist;
//         break;
//     }
//     seen.insert((y_coord, x_coord));
//     // get left adj
//     if x_coord > 0 {
//         if !seen.contains(&(y_coord, x_coord - 1)) {
//             stack.push_back((y_coord, x_coord - 1, dist + 1));
//         }
//     }
//     // get right adj
//     if x_coord < x_size - 1 {
//         if !seen.contains(&(y_coord, x_coord + 1)) {
//             stack.push_back((y_coord, x_coord + 1, dist + 1));
//         }
//     }
//     // get top adj
//     if y_coord > 0 {
//         if !seen.contains(&(y_coord - 1, x_coord)) {
//             stack.push_back((y_coord - 1, x_coord, dist + 1));
//         }
//     }
//     // get bottom adj
//     if y_coord < y_size - 1 {
//         if !seen.contains(&(y_coord + 1, x_coord)) {
//             stack.push_back((y_coord + 1, x_coord, dist + 1));
//         }
//     }
// }
// }
// mat_map
// }

#[cfg(test)]
#[test]
fn test_1() {
    let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];

    assert_eq!(update_matrix(mat), expected);
}

#[test]
fn test_2() {
    let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    let expected = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];
    assert_eq!(update_matrix(mat), expected);
}

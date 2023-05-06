//An image is represented by an m x n integer grid image where image[i][j] represents the pixel value of the image.

//You are also given three integers sr, sc, and color. You should perform a flood fill on the image starting from the pixel image[sr][sc].

//To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color), and so on. Replace the color of all of the aforementioned pixels with color.

//Return the modified image after performing the flood fill.

fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    // Set vars

    let height = image.len();
    let width = if image.len() == 0 {
        0
    } else {
        image.last().as_ref().unwrap().len()
    };

    //Edge cases

    if width == 0 || height == 0 || image[sr as usize][sc as usize] == color {
        return image;
    }

    // init stack
    let mut stack = Vec::new();
    let start_color = image[sr as usize][sc as usize];
    stack.push((sr as usize, sc as usize));

    while let Some((current_r, current_c)) = stack.pop() {
        //  change color
        image[current_r][current_c] = color;
        // add neighbors of same color to stack
        // Y
        let lower_bound = 0.max(current_r as i32 - 1) as usize;
        let upper_bound = height.min(current_r + 2);
        for y in lower_bound..upper_bound {
            if image[y][current_c] == start_color {
                stack.push((y, current_c))
            }
        }
        // X
        let lower_bound = 0.max(current_c as i32 - 1) as usize;
        let upper_bound = (width).min(current_c + 2);
        for x in lower_bound..upper_bound {
            if image[current_r][x] == start_color {
                stack.push((current_r as usize, x));
            }
        }
    }
    image
}

#[cfg(test)]
#[test]
fn given_1() {
    let (image, sr, sc, color) = (vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2);
    let expected = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];

    assert_eq!(flood_fill(image, sr, sc, color), expected);
}

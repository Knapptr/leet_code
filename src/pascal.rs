pub fn pascal(n_rows: i32) -> Vec<Vec<i32>> {
    if n_rows == 0 {
        return vec![];
    }
    if n_rows == 1 {
        return vec![vec![1]];
    };
    if n_rows == 2 {
        let mut vec = pascal(1);
        vec.push(vec![1, 1]);
        return vec;
    }
    let mut vec = pascal(n_rows - 1);
    let last_row = vec.last().unwrap();
    let mut current_row = vec![1];
    for i in 1..n_rows - 1 {
        current_row
            .push(last_row.get(i as usize - 1usize).unwrap() + last_row.get(i as usize).unwrap());
    }
    current_row.push(1);
    vec.push(current_row);
    vec
}

#[cfg(test)]
#[test]
fn first_row() {
    assert_eq!(pascal(1), vec![[1]]);
}

#[test]
fn second_row() {
    assert_eq!(pascal(2), vec![vec![1], vec![1, 1]]);
}

#[test]
fn third_row() {
    assert_eq!(pascal(3), vec![vec![1], vec![1, 1], vec![1, 2, 1]]);
}

#[test]
fn fourth_row() {
    assert_eq!(
        pascal(4),
        vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]
    );
}

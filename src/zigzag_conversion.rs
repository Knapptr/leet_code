// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

// P   A   H   N
// A P L S I I G
// Y   I   R
// And then read line by line: "PAHNAPLSIIGYIR"

// Write the code that will take a string and make this conversion given a number of rows:

// string convert(string s, int numRows);

fn convert(s: String, num_rows: i32) -> String {
    let mut cur_row = 0usize;
    let mut zig = true;
    let mut rows = vec![String::new(); num_rows as usize];
    for char in s.chars() {
        rows[cur_row].push(char);
        if zig {
            cur_row += 1;
            if cur_row == num_rows as usize {
                zig = false;
                cur_row -= 1;
            }
        }
        if !zig {
            cur_row = cur_row - 1;
            if cur_row == 0 {
                zig = true;
            }
        }
    }
    rows.join("").to_string()
}

#[cfg(test)]
#[test]
fn example_1() {
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 3;

    let expected = "PAHNAPLSIIGYIR".to_string();

    assert_eq!(convert(s, num_rows), expected);
}

#[test]
fn example_2() {
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 4;
    let expected = "PINALSIGYAHRPI".to_string();

    assert_eq!(convert(s, num_rows), expected);
}

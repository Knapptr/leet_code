// a stub version of the 'is bad version' api provided by leetcode
fn mock_is_bad_version(first_bad: i32) -> Box<dyn Fn(i32) -> bool> {
    Box::new(move |n: i32| -> bool {
        if n == first_bad {
            true
        } else {
            false
        }
    })
}
pub fn first_bad_version(n: i32, answer: i32) -> i32 {
    let mut lo = 0;
    let mut hi = n;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if mock_is_bad_version(answer)(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

#[cfg(test)]
#[test]
fn case_1() {
    assert_eq!(first_bad_version(5, 4), 4)
}

#[test]
fn case_2() {
    assert_eq!(first_bad_version(1, 1), 1)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn from_vec(vals: Vec<i32>) -> Self {
        let mut dummy_head = ListNode::new(-1);
        let mut current_val = &mut dummy_head;
        for val in vals {
            current_val.next = Some(Box::new(ListNode::new(val)));
            current_val = current_val.next.as_mut().unwrap()
        }
        *dummy_head.next.unwrap()
    }
}
pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut fast = head.clone();
    let mut slow = &mut head;

    //iterate fast up by n
    for _ in 0..n {
        if let Some(f) = fast {
            fast = f.next
        }
    }

    // move slow and fast up by n
    loop {
        match fast {
            Some(f) => fast = f.next,
            None => return head.unwrap().next,
        }
        if fast.is_none() {
            break;
        }
        match slow {
            Some(s) => slow = &mut s.next,
            None => panic!("Something went very wrong"),
        }
    }
    // make changes
    match slow {
        Some(s) => s.next = s.next.as_mut().unwrap().next.take(),
        None => panic!("Something went very wrong"),
    };

    head
}

#[cfg(test)]
#[test]
fn short() {
    let head = Some(Box::new(ListNode::from_vec(vec![1, 2, 3, 4, 5])));
    let n = 2;
    let expected = Some(Box::new(ListNode::from_vec(vec![1, 2, 3, 5])));

    assert_eq!(remove_nth_from_end(head, n), expected);
}

#[test]
fn shorter_1() {
    let head = Some(Box::new(ListNode::from_vec(vec![1, 2])));
    let n = 1;
    let expected = Some(Box::new(ListNode::from_vec(vec![1])));

    assert_eq!(remove_nth_from_end(head, n), expected);
}
#[test]
fn shorter_2() {
    let head = Some(Box::new(ListNode::from_vec(vec![1, 2])));
    let n = 2;
    let expected = Some(Box::new(ListNode::from_vec(vec![2])));

    assert_eq!(remove_nth_from_end(head, n), expected);
}
#[test]
fn none() {
    let head = Some(Box::new(ListNode::from_vec(vec![1])));
    let n = 1;
    let expected = None;

    assert_eq!(remove_nth_from_end(head, n), expected);
}

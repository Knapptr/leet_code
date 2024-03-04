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
    if head.is_none() {
        return None;
    };
    let mut dummy = Some(ListNode {
        val: -1,
        next: head.clone(),
    });
    // setup end pointer
    let mut end = head.unwrap();
    // move end pointer n forward
    for _ in 1..n {
        match end.next {
            Some(n) => end = n,
            None => panic!(),
        }
    }
    let mut start = dummy.as_mut().unwrap();
    // move start and and 1 at a time
    while end.next.is_some() {
        end = end.next.take().unwrap();
        start = start.next.as_mut().unwrap();
    }
    start.next = start.next.as_mut().unwrap().next.take();
    dummy.unwrap().next

    // when end pointer reaches end, set start.next to start.next.next
}

#[cfg(test)]
#[test]
fn example_one() {
    let head = Some(Box::new(ListNode::from_vec(vec![1, 2, 3, 4, 5])));
    let n = 2;
    let expected = Some(Box::new(ListNode::from_vec(vec![1, 2, 3, 5])));

    assert_eq!(remove_nth_from_end(head, n), expected);
}

#[test]
fn example_two() {
    let head = Some(Box::new(ListNode::from_vec(vec![1])));
    let n = 1;
    let expected = None;

    assert_eq!(remove_nth_from_end(head, n), expected);
}

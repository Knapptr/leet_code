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

fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // create dummy head
    // set dummy head as predec
    let mut dummy = Box::new(ListNode {
        val: -1,
        next: head,
    });
    let mut prec = &mut dummy;
    // set current node to predec.next if it has one
    if prec.next.is_none() {
        return None;
    }
    let mut current = prec.next.as_ref().unwrap();
    // check next of current_node
    while current.next.is_some() {
        while current.next.as_ref().unwrap().val == current.val {
            let n = current.next.as_mut().take().unwrap();
            prec.next = *n;
            current = prec.next.as_mut().unwrap();
        }
        // if values !=, move predec to pred.next
        prec = prec.next.as_mut().unwrap();
    }
    dummy.next
}

#[cfg(test)]
#[test]
fn example_one() {
    let nums = ListNode::from_vec(vec![1, 2, 3, 3, 4, 5]);
    let expected = Box::new(ListNode::from_vec(vec![1, 2, 5]));

    assert_eq!(delete_duplicates(Some(Box::new(nums))).unwrap(), expected);
}

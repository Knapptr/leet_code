// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // // SOLUTION TWO
    // // Use 2 pointers
    if let None = head {
        return None;
    };

    let mut slow = &head;
    let mut fast = &head;

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        slow = &slow.as_ref().unwrap().next;
    }
    slow.clone()

    // // SOLUTION ONE
    // // non 2 pointer but count nodes (no mem alloc)
    // // get length
    // let mut count = 0;
    // let mut counter_node = &head;
    // while let Some(c) = counter_node {
    //     count += 1;
    //     counter_node = &c.next;
    // }

    // if count == 0 {
    //     None
    // } else {
    //     println!("Count: {count}");
    //     for _ in 0..(count / 2) {
    //         head = head.unwrap().next
    //     }
    //     head
    // }
}

#[cfg(test)]
#[test]
fn odd() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut current_node = head.as_mut().unwrap();
    current_node.next = Some(Box::new(ListNode::new(2)));
    current_node = current_node.next.as_mut().unwrap();
    current_node.next = Some(Box::new(ListNode::new(3)));
    current_node = current_node.next.as_mut().unwrap();
    current_node.next = Some(Box::new(ListNode::new(4)));
    current_node = current_node.next.as_mut().unwrap();
    current_node.next = Some(Box::new(ListNode::new(5)));

    let expected = Some(
        head.as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .clone(),
    );

    assert_eq!(middle_node(head), expected);
}

#[test]
fn even() {
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut current_node = head.as_mut().unwrap();
    current_node.next = Some(Box::new(ListNode::new(2)));
    current_node = current_node.next.as_mut().unwrap();
    current_node.next = Some(Box::new(ListNode::new(3)));
    current_node = current_node.next.as_mut().unwrap();
    current_node.next = Some(Box::new(ListNode::new(4)));
    current_node = current_node.next.as_mut().unwrap();
    current_node.next = Some(Box::new(ListNode::new(5)));
    current_node = current_node.next.as_mut().unwrap();
    current_node.next = Some(Box::new(ListNode::new(6)));

    let expected = Some(
        head.as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .next
            .as_ref()
            .unwrap()
            .clone(),
    );

    assert_eq!(middle_node(head), expected);
}

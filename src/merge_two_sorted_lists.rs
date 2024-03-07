// You are given the heads of two sorted linked lists list1 and list2.

// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

// Return the head of the merged linked list.
//
//
// Definition for singly-linked list.
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
}
pub struct Solution;
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut pre_head = ListNode::new(0);
        let mut current_node = &mut pre_head;

        loop {
            match (list1, list2) {
                (None, None) => break,
                (Some(l), None) | (None, Some(l)) => {
                    current_node.next = Some(l);
                    break;
                }
                (Some(l1), Some(l2)) => {
                    // compare values
                    match l1.val.cmp(&l2.val) {
                        std::cmp::Ordering::Greater => {
                            // Make current value take on the value of list 2
                            current_node.next = Some(Box::new(ListNode::new(l2.val)));
                            // move new currentvalue to list2
                            list2 = l2.next;
                            list1 = Some(l1);
                            current_node = current_node.next.as_mut().unwrap();
                            // todo!();
                        }
                        std::cmp::Ordering::Less => {
                            // Make current value take on the value of list 1
                            current_node.next = Some(Box::new(ListNode::new(l1.val)));
                            // move new currentvalue to list1
                            list1 = l1.next;
                            list2 = Some(l2);
                            current_node = current_node.next.as_mut().unwrap();
                            // todo!();
                        }
                        _ => {
                            // Make current value take on the value of list 1
                            current_node.next = Some(Box::new(ListNode::new(l1.val)));
                            // move new currentvalue to list1
                            list1 = l1.next;
                            list2 = Some(l2);
                            current_node = current_node.next.as_mut().unwrap();
                            // todo!();
                        }
                    }
                }
            }
        }
        pre_head.next
    }
}

#[cfg(test)]
#[test]
fn compiles() {
    assert!(true)
}

#[test]
fn handle_all_none() {
    let solution = Solution::merge_two_lists(None, None);
    assert!(matches!(solution, None))
}

#[test]
fn handle_one_none() {
    let list = Some(Box::new(ListNode::new(1)));
    assert_eq!(list.clone(), Solution::merge_two_lists(list, None));

    let list = Some(Box::new(ListNode::new(1)));
    assert_eq!(list.clone(), Solution::merge_two_lists(None, list));
}

#[test]
fn merge_test_single() {
    let list_1 = Some(Box::new(ListNode::new(1)));
    let list_2 = Some(Box::new(ListNode::new(2)));

    let mut expected_list = Box::new(ListNode::new(1));
    expected_list.next = Some(Box::new(ListNode::new(2)));

    let merged_list = Solution::merge_two_lists(list_1, list_2).unwrap();

    assert_eq!(merged_list, expected_list);
}

#[test]
fn merge_test_equal_2() {
    let list_1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode { val: 4, next: None })),
    }));
    let list_2 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode { val: 3, next: None })),
    }));

    let expected_list = Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    });

    let merged_list = Solution::merge_two_lists(list_1, list_2).unwrap();

    assert_eq!(merged_list, expected_list);
}
#[test]
fn merge_test_not_equal_2() {
    let list_1 = Some(Box::new(ListNode { val: 1, next: None }));
    let list_2 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode { val: 3, next: None })),
    }));

    let expected_list = Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    });

    let merged_list = Solution::merge_two_lists(list_1, list_2).unwrap();

    assert_eq!(merged_list, expected_list);
}

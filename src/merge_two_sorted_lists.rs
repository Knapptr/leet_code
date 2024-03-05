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
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(l)) => Some(l),
            (Some(l1), Some(l2)) => {
                let mut p1 = l1;
                let mut p2 = l2;
                let mut head: Option<Box<ListNode>> = None;

                // Compare values
                // Put lower value onto new list
                // move lower value OG list to next value
                head
            }
        }
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
fn merge_test() {
    let list_1 = Some(Box::new(ListNode::new(1)));
    let list_2 = Some(Box::new(ListNode::new(2)));

    let mut expected_list = Box::new(ListNode::new(1));
    expected_list.next = Some(Box::new(ListNode::new(2)));

    let merged_list = Solution::merge_two_lists(list_1, list_2).unwrap();

    assert_eq!(merged_list, expected_list);
}

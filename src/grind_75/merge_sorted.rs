use crate::solution::Solution;

/*You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list. */

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
impl From<Vec<i32>> for ListNode {
    fn from(vec: Vec<i32>) -> Self {
        let mut root = ListNode::new(-69);
        let mut current_value = &mut root;
        for num in vec {
            let new_node = ListNode::new(num);
            current_value.next.replace(Box::new(new_node));
            current_value = current_value.next.as_mut().unwrap();
        }
        return *root.next.unwrap();
    }
}
impl From<ListNode> for Vec<i32> {
    fn from(mut list_node: ListNode) -> Self {
        let mut vec = vec![];
        vec.push(list_node.val);
        while let Some(next) = list_node.next {
            vec.push(next.val);
            list_node = *next;
        }
        return vec;
    }
}
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut new_list = ListNode::new(-69);
        let mut current_value = &mut new_list;
        loop {
            match (list1, list2) {
                (None, None) => return new_list.next,
                (Some(l), None) => {
                    current_value.next.replace(l);
                    return new_list.next;
                }
                (None, Some(r)) => {
                    current_value.next.replace(r);
                    return new_list.next;
                }
                (Some(l), Some(r)) => {
                    // compare l and r
                    if l.val < r.val {
                        current_value.next.replace(l);
                        current_value = current_value.next.as_mut().unwrap();
                        // reassign list1
                        list1 = current_value.next.take();
                        list2 = Some(r);
                    } else {
                        current_value.next.replace(r);
                        current_value = current_value.next.as_mut().unwrap();
                        // reassign list1
                        list2 = current_value.next.take();
                        list1 = Some(l);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn creation_and_into() {
    let list1 = ListNode::from(vec![1, 2, 4]);
    let expected = vec![1, 2, 4];
    assert_eq!(Vec::from(list1), expected);
}
#[test]
fn example_one() {
    let list1 = Some(Box::new(ListNode::from(vec![1, 2, 4])));
    let list2 = Some(Box::new(ListNode::from(vec![1, 3, 4])));
    let expected = vec![1, 1, 2, 3, 4, 4];
    assert_eq!(
        Vec::from(*Solution::merge_two_lists(list1, list2).unwrap()),
        expected
    );
}

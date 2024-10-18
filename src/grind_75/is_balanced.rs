use crate::solution::Solution;

/*Given a binary tree, determine if it is
height-balanced.

A height-balanced binary tree is a binary tree in which the depth of the two subtrees of every node never differs by more than one.
*/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // recurse and return depth
        fn recurse(root: Option<&Rc<RefCell<TreeNode>>>) -> Option<i32> {
            if root.is_none() {
                return Some(0);
            }

            // Recurse and return the depth of the subtree of each child
            let current_node = root.as_ref().cloned().unwrap();
            // return 0 if both children are none (end of the line)

            // get balance factor, if abs greater than 1, return err
            if let Some(left_depth) = recurse(current_node.borrow().left.as_ref()) {
                if let Some(right_depth) = recurse(current_node.borrow().right.as_ref()) {
                    let factor = (left_depth - right_depth).abs();
                    if factor <= 1 {
                        return Some(left_depth.max(right_depth) + 1);
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
            // Return 1 + max depth of subtree
            return None;
        }
        if root.is_none() {
            return true;
        };
        match recurse(root.as_ref()) {
            None => false,
            Some(_v) => true,
        }
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let tree = TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    };

    assert!(Solution::is_balanced(Some(Rc::new(RefCell::new(tree)))));
}

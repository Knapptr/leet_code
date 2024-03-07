// Given the root of a binary tree, return its maximum depth.

// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
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

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root_node = root.unwrap();
        let mut stack = vec![];
        let mut max_depth = 0;

        stack.push((root_node.clone(), 1));

        while stack.len() > 0 {
            let (current_node, current_depth) = stack.pop().unwrap();
            max_depth = max_depth.max(current_depth);
            if current_node.borrow().left.is_some() {
                stack.push((
                    current_node.borrow().left.as_ref().unwrap().clone(),
                    current_depth + 1,
                ))
            }
            if current_node.borrow().right.is_some() {
                stack.push((
                    current_node.borrow().right.as_ref().unwrap().clone(),
                    current_depth + 1,
                ))
            }
        }

        max_depth
    }
}

#[cfg(test)]
#[test]
fn compiles() {}

#[test]
fn example_one() {
    let tree = TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    };

    assert_eq!(Solution::max_depth(Some(Rc::new(RefCell::new(tree)))), 3);
}

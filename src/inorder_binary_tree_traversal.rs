/*
Given the root of a binary tree, return the inorder traversal of its nodes' values.
*/
// Definition for a binary tree node.
struct Solution {}
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // recursive helper
        fn recurse(root: &Option<Rc<RefCell<TreeNode>>>, accumulator: Vec<i32>) -> Vec<i32> {
            // if root is none, return vector
            if root.is_none() {
                return accumulator;
            } else {
                // DFS add left to array, then add right
                let current_node = root.as_ref().unwrap();
                // if has left recurse on that node, if not, add value to array and move on
                // Do left
                let mut accumulated_left = recurse(&current_node.borrow().left, accumulator);
                // Add value
                accumulated_left.push(current_node.borrow().val);
                // Do right
                let accumulated_all = recurse(&current_node.borrow().right, accumulated_left);
                return accumulated_all;
            }
        }
        let result = Vec::new();
        return recurse(&root, result);
    }
}

#[cfg(test)]
#[test]
fn test_one() {
    let mut root = TreeNode::new(4);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    let expected = vec![2, 4, 6];
    assert_eq!(
        Solution::inorder_traversal(Some(Rc::new(RefCell::new(root)))),
        expected
    );
}

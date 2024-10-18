// Given a binary tree, find its minimum depth.

// The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.

// Note: A leaf is a node with no children.

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
impl From<Vec<i32>> for TreeNode {
    fn from(vector: Vec<Option<i32>>) -> Self {
        let mut v_iter = vector.iter().enumerate();
        while let Some((index, current_value)) = v_iter.next() {
            if let Some(value) = current_value{
                let node = TreeNode::new(*value);
                let li = (2*index + 1);
                let ri = (2*index + 2);
                node.left = vector[li].clone();
            }
            if index + 2
        }
        Self {}
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
#[test]
fn test_one() {
    println!("pass");
}

use crate::solution::Solution;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
/*Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.

According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”*/

/*
First thoughts (This might work, but it's easier- see below)
    Use 2 hashmaps to log the parent of a nodes (stop building the hash map when you reach the node)

    starting with either node, step backwards through parents until an entry exists in the other hash map
*/

/* WAIT! an inspiration!.
Descend the tree until there is a deviation- that node is the LCA! */

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
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        let root = root.unwrap();
        //descend until deviation
        let mut current_node = root.clone();
        loop {
            let current_val = current_node.borrow().val;
            if current_val == p_val {
                return Some(current_node);
            }
            if current_val == q_val {
                return Some(current_node);
            }
            if p_val > current_val && q_val > current_val {
                current_node = current_node
                    .clone()
                    .borrow()
                    .right
                    .as_ref()
                    .cloned()
                    .unwrap();
                continue;
            }
            if p_val < current_val && q_val < current_val {
                current_node = current_node
                    .clone()
                    .borrow()
                    .left
                    .as_ref()
                    .cloned()
                    .unwrap();
                continue;
            }
            break;
        }
        Some(current_node)
    }
}

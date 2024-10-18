use crate::solution::Solution;
/*
Given the root of a binary tree, invert the tree, and return its root.
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
    pub fn insert(&mut self, val: i32) {
        // compare value
        if val > self.val {
            match &self.right {
                None => {
                    self.right
                        .replace(Rc::new(RefCell::new(TreeNode::new(val))));
                }
                Some(node) => {
                    node.borrow_mut().insert(val);
                }
            }
        } else {
            match &self.left {
                None => {
                    self.left.replace(Rc::new(RefCell::new(TreeNode::new(val))));
                }
                Some(node) => {
                    node.borrow_mut().insert(val);
                }
            }
        }
    }
}

impl From<Vec<i32>> for TreeNode {
    // don't bother with a lot of logic here, just make a simple insert method and iterate the entire thing. This is just for tests!
    fn from(vec: Vec<i32>) -> Self {
        let mut root = TreeNode::new(vec[0]);
        for value in vec.iter().skip(1) {
            root.insert(*value);
        }
        root
    }
}
impl From<TreeNode> for Vec<i32> {
    fn from(root: TreeNode) -> Self {
        let mut vec = Vec::new();
        // bfs
        let mut queue = VecDeque::new();
        queue.push_back(Rc::new(RefCell::new(root)));
        loop {
            println!("Queue: {:?}", queue);
            if queue.len() == 0 {
                break;
            }
            // pop off of queue
            let current = queue.pop_front().unwrap();
            let cv = current.borrow();

            vec.push(cv.val);
            match (cv.left.as_ref(), cv.right.as_ref()) {
                (None, None) => {
                    continue;
                }
                (Some(n), None) => {
                    queue.push_back(n.clone());
                }
                (None, Some(n)) => {
                    queue.push_back(n.clone());
                }
                (Some(l), Some(r)) => {
                    queue.push_back(l.clone());
                    queue.push_back(r.clone());
                }
            };
        }
        return vec;
    }
}
use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn recurse(node: Rc<RefCell<TreeNode>>) {
            // base case: node has no children- return
            let mut current_node = node.borrow_mut();
            match (&current_node.left, &current_node.right) {
                (None, None) => {
                    return;
                }
                (Some(l), None) => recurse(l.clone()),
                (None, Some(r)) => recurse(r.clone()),
                (Some(l), Some(r)) => {
                    recurse(l.clone());
                    recurse(r.clone())
                }
            }
            // swap left and right
            let temp_l = current_node.left.take();
            let temp_r = current_node.right.take();
            match (temp_l, temp_r) {
                (Some(l), None) => {
                    current_node.right.replace(l);
                }
                (None, None) => (),
                (None, Some(r)) => {
                    current_node.left.replace(r);
                }
                (Some(l), Some(r)) => {
                    current_node.left.replace(r);
                    current_node.right.replace(l);
                }
            }
        }
        match root {
            None => None,
            Some(r) => {
                recurse(r.clone());
                return Some(r);
            }
        }
    }
    pub fn invert_tree_2nd_attempt(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let mut mutable_node = node.borrow_mut();
            // Call recursively on left
            Solution::invert_tree_2nd_attempt(mutable_node.left.as_ref().cloned());
            Solution::invert_tree_2nd_attempt(mutable_node.right.as_ref().cloned());
            //swap left and right
            let temp = mutable_node.left.take();
            mutable_node.left = mutable_node.right.take();
            mutable_node.right = temp;
        }
        return root;
    }
}

#[cfg(test)]
#[test]
fn from_vec() {
    let expected = TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
    };
    let vec = vec![4, 3, 7];
    assert_eq!(TreeNode::from(vec), expected);
}

#[test]
fn tree_to_vec() {
    let tree = TreeNode::from(vec![4, 3, 7, 1, 5]);
    let expected = vec![4, 3, 7, 1, 5];
    println!("Tree: {:?}", tree);
    assert_eq!(Vec::from(tree), expected);
}

#[test]
fn example_one() {
    let mut tree = Some(Rc::new(RefCell::new(TreeNode::from(vec![
        4, 2, 7, 1, 3, 6, 9,
    ]))));
    let expected = vec![4, 7, 2, 9, 6, 3, 1];
    let inverted = Solution::invert_tree_2nd_attempt(tree).unwrap();
    let taken = Rc::try_unwrap(inverted).unwrap().into_inner();
    let raw_tree: Vec<i32> = Vec::from(taken);
    assert_eq!(raw_tree, expected);
}

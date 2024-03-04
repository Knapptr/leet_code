// Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.
//
//A leaf is a node with no children.
//
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
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    let mut stack = Vec::new();

    stack.push((root, 0));

    while stack.len() > 0 {
        let (current, sum) = stack.pop().expect("Something went wrong");
        match current {
            Some(node) => {
                let val = node.borrow().val;
                if val + sum == target_sum {
                    return true;
                }
                stack.push((node.borrow_mut().left.take(), val + sum));
                stack.push((node.borrow_mut().right.take(), val + sum));
            }
            None => {
                continue;
            }
        }
    }
    false
}

#[cfg(test)]
#[test]
fn example_two() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
    })));

    let expected = false;
    let target_sum = 5;

    assert_eq!(has_path_sum(root, target_sum), expected);
}
#[test]
fn example_one() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
    })));

    let expected = true;
    let target_sum = 4;

    assert_eq!(has_path_sum(root, target_sum), expected);
}

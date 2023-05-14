// You are given two binary trees root1 and root2.
//
// Imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not. You need to merge the two trees into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of the new tree.
//
// Return the merged tree.
//
// Note: The merging process must start from the root nodes of both trees.
//
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Tree>,
    pub right: Option<Tree>,
}

type Tree = Rc<RefCell<TreeNode>>;

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

pub fn insert_pointer(root: &mut Option<Tree>, val: i32) {
    let mut current_node = match root {
        Some(node) => node.clone(),
        None => {
            *root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            return;
        }
    };

    ////USING POINTER?
    loop {
        let node_value = current_node.borrow().val;

        if val < node_value {
            if current_node.borrow().left.is_none() {
                current_node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                break;
            } else {
                let n = current_node.borrow().left.as_ref().unwrap().clone();
                current_node = n;
            }
        } else {
            if current_node.borrow().right.is_none() {
                current_node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                break;
            } else {
                let n = current_node.borrow().right.as_ref().unwrap().clone();
                current_node = n;
            }
        }
    }
}
pub fn insert_stack(root: &mut Option<Tree>, val: i32) {
    let mut current_node = match root {
        Some(node) => node.clone(),
        None => {
            *root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            return;
        }
    };
    //// USING STACK
    let mut stack: Vec<Tree> = Vec::new();
    stack.push(current_node.clone());
    while let Some(node) = stack.pop() {
        let mut node = node.borrow_mut();
        if val < node.val {
            match &node.left {
                None => node.left = Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                Some(n) => stack.push(n.clone()),
            }
        } else {
            match &node.right {
                None => node.right = Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                Some(n) => stack.push(n.clone()),
            }
        }
    }
}

fn merge_nodes(node1: Option<&Tree>, node2: Option<&Tree>) -> Option<Tree> {
    let mut new_node = Rc::new(RefCell::new(TreeNode::new(-1)));
    match (node1, node2) {
        (None, None) => return None,
        (Some(_), None) => return Some(node1.unwrap().clone()),
        (None, Some(_)) => return Some(node2.unwrap().clone()),
        (Some(t1), Some(t2)) => new_node.borrow_mut().val = t1.borrow().val + t2.borrow().val,
    }
    Some(new_node)
}
fn merge_trees(root1: Option<Tree>, root2: Option<Tree>) -> Option<Tree> {
    let mut tree_1 = root1.as_ref();
    let mut tree_2 = root2.as_ref();

    let mut merged_tree = Rc::new(RefCell::new(TreeNode::new(0)));
    let mut current_node = merged_tree.clone();

    // merge current node
    // current_node.borrow_mut().val = tree_1.borrow().val + tree_2.borrow().val;
    // merge left node
    // merge right node

    todo!()
}

// fn dfs_print(root: Tree) {
//     let mut vals = Vec::new();
// }

#[cfg(test)]
// #[test]
// fn merge_two_nodes() {
//     let root1 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
//     let root2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));

//     let merged = merge_nodes(&root1, &root2).unwrap();

//     assert_eq!(merged.borrow().val, 4);
// }
#[test]
fn merge_simple_tree() {
    let mut tree1 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    insert_pointer(&mut tree1, 3);
    insert_pointer(&mut tree1, 5);
    let mut tree2 = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    insert_pointer(&mut tree2, 3);
    insert_pointer(&mut tree2, 5);

    let merged = merge_trees(tree1, tree2).unwrap();

    assert_eq!(merged.borrow().val, 8);
    assert_eq!(merged.borrow().left.as_ref().unwrap().borrow().val, 6);
    assert_eq!(merged.borrow().right.as_ref().unwrap().borrow().val, 10);
}

#[test]
fn insertion() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    insert_pointer(&mut root, 6);
    insert_pointer(&mut root, 4);

    let left_val = root
        .as_ref()
        .unwrap()
        .borrow()
        .left
        .as_ref()
        .unwrap()
        .borrow()
        .val;
    let right_val = root
        .as_ref()
        .unwrap()
        .borrow()
        .right
        .as_ref()
        .unwrap()
        .borrow()
        .val;

    assert_eq!(left_val, 4);
    assert_eq!(right_val, 6);
}

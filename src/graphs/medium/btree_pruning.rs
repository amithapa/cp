/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 814. Binary Tree Pruning
Link: https://leetcode.com/problems/binary-tree-pruning/
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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root.clone() {
            Some(root_ref) => {
                let mut root_node = root_ref.borrow_mut();

                root_node.left = Self::prune_tree(root_node.left.take());
                root_node.right = Self::prune_tree(root_node.right.take());

                if root_node.left.is_none() && root_node.right.is_none() && root_node.val == 0 {
                    None
                } else {
                    root
                }
            }
            None => root,
        }
    }
}

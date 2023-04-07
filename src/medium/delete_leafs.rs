/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1325. Delete Leaves With a Given Value
Link: https://leetcode.com/problems/delete-leaves-with-a-given-value/
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
struct Solution;

impl Solution {
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root.clone() {
            Some(root_ref) => {
                let mut root_node = root_ref.borrow_mut();

                root_node.left = Self::remove_leaf_nodes(root_node.left.take(), target);
                root_node.right = Self::remove_leaf_nodes(root_node.right.take(), target);

                if root_node.left.is_none() && root_node.right.is_none() && root_node.val == target
                {
                    None
                } else {
                    root
                }
            }
            None => None,
        }
    }
}

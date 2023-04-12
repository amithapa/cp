/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 230. Kth Smallest Element in a BST
Link: https://leetcode.com/problems/kth-smallest-element-in-a-bst/
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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
        if let Some(root_ref) = root {
            let root_node = root_ref.borrow();
            Self::dfs(&root_node.left, order);
            order.push(root_node.val);
            Self::dfs(&root_node.right, order);
        }
    }
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut order: Vec<i32> = Vec::new();
        Self::dfs(&root, &mut order);
        order[k as usize - 1]
    }
}

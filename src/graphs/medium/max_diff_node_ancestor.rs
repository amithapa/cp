/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1026. Maximum Difference Between Node and Ancestor
Link: https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/description/
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
    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, max_number: i32, min_number: i32) -> i32 {
        match root {
            Some(root_ref) => {
                let root_node = root_ref.borrow();
                let value = root_node.val;

                let max_number = max_number.max(value);
                let min_number = min_number.min(value);

                Self::dfs(&root_node.left, max_number, min_number).max(Self::dfs(
                    &root_node.right,
                    max_number,
                    min_number,
                ))
            }
            None => max_number - min_number,
        }
    }

    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, i32::MIN, i32::MAX)
    }
}

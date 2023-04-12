/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 979. Distribute Coins in Binary Tree
Link: https://leetcode.com/problems/distribute-coins-in-binary-tree/
 */

// Definition for a binary tree node.

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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, moves: &mut i32) -> i32 {
        match root {
            Some(root_ref) => {
                let root_node = root_ref.borrow();

                let left = Self::dfs(&root_node.left, moves);
                let right = Self::dfs(&root_node.right, moves);

                *moves += left.abs() + right.abs();

                root_node.val + left + right - 1
            }
            None => 0,
        }
    }
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut moves = 0;
        Self::dfs(&root, &mut moves);
        moves
    }
}

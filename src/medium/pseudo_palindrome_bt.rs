/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1457. Pseudo-Palindromic Paths in a Binary Tree
Link: https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/description/
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

use std::collections::HashSet;
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, occurrences: &mut HashSet<i32>, count: &mut i32) {
        if let Some(root_ref) = root {
            let root_node = root_ref.borrow();
            let contains_number = occurrences.contains(&root_node.val);

            match contains_number {
                true => occurrences.remove(&root_node.val),
                false => occurrences.insert(root_node.val),
            };

            if root_node.left.is_none() && root_node.right.is_none() {
                if occurrences.len() <= 1 {
                    *count += 1;
                }
            } else {
                Self::dfs(&root_node.left, occurrences, count);
                Self::dfs(&root_node.right, occurrences, count);
            }

            match contains_number {
                true => occurrences.insert(root_node.val),
                false => occurrences.remove(&root_node.val),
            };
        }
    }

    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count: i32 = 0;
        Self::dfs(&root, &mut HashSet::new(), &mut count);
        count
    }
}

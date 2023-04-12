/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 865. Smallest Subtree with all the Deepest Nodes
Link: https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/
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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        match root.clone() {
            Some(root_ref) => {
                let root_node = root_ref.borrow();

                let (left_height, left_tree) = Self::dfs(&root_node.left);
                let (right_height, right_tree) = Self::dfs(&root_node.right);

                if left_height > right_height {
                    (left_height + 1, left_tree)
                } else if right_height > left_height {
                    (right_height + 1, right_tree)
                } else {
                    (left_height + 1, root.clone())
                }
            }
            None => (0, None),
        }
    }

    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root).1
    }
}

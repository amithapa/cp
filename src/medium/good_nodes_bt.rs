/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1448. Count Good Nodes in Binary Tree
Link: https://leetcode.com/problems/count-good-nodes-in-binary-tree/
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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, node_value: i32, count: &mut i32) {
        if let Some(root_ref) = root {
            let root_node = root_ref.borrow();
            if root_node.val >= node_value {
                *count += 1;
            }
            let node_value = node_value.max(root_node.val);
            Self::dfs(&root_node.left, node_value, count);
            Self::dfs(&root_node.right, node_value, count);
        }
    }

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        Self::dfs(&root, i32::MIN, &mut count);
        count
    }
}

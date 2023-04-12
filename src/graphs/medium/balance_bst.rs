/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1382. Balance a Binary Search Tree
Link: https://leetcode.com/problems/balance-a-binary-search-tree/
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

type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    fn dfs(root: &Node, order: &mut Vec<i32>) {
        if let Some(root_ref) = root {
            let root_node = root_ref.borrow();
            Self::dfs(&root_node.left, order);
            order.push(root_node.val);
            Self::dfs(&root_node.right, order);
        }
    }

    fn construct_bst(order: &[i32], start: usize, end: usize) -> Node {
        if start < end {
            let root_index = (start + end) / 2;

            let mut root = TreeNode::new(order[root_index]);
            root.left = Self::construct_bst(order, start, root_index);
            root.right = Self::construct_bst(order, root_index + 1, end);

            Some(Rc::new(RefCell::new(root)))
        } else {
            None
        }
    }

    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Node {
        let mut order: Vec<i32> = Vec::new();
        Self::dfs(&root, &mut order);

        Self::construct_bst(&order, 0, order.len())
    }
}

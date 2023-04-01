/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 2265. Count Nodes Equal to Average of Subtree
Link: https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/
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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> (i32, i32) {
        match root {
            Some(root_ref) => {
                let root_node = root_ref.borrow();
                let (left_total, left_count) = Self::dfs(&root_node.left, count);
                let (right_total, right_count) = Self::dfs(&root_node.right, count);

                let current_subtree_sum = root_node.val + left_total + right_total;
                let current_node_count = left_count + right_count + 1;

                let average = current_subtree_sum / current_node_count;
                if average == root_node.val {
                    *count += 1;
                }

                (current_subtree_sum, current_node_count)
            }
            None => (0, 0),
        }
    }

    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        Self::dfs(&root, &mut count);
        count
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{Solution, TreeNode};

    fn construct_tree(data: &[Option<i32>], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if index < data.len() {
            if let Some(value) = data[index] {
                let mut node = TreeNode::new(value);
                node.left = construct_tree(data, index * 2 + 1);
                node.right = construct_tree(data, index * 2 + 2);
                Some(Rc::new(RefCell::new(node)))
            } else {
                None
            }
        } else {
            None
        }
    }

    #[test]
    fn test_case_i() {
        let input_data = [Some(4), Some(8), Some(5), Some(0), Some(1), None, Some(6)];

        let root_node: Option<Rc<RefCell<TreeNode>>> = construct_tree(&input_data, 0);

        assert_eq!(Solution::average_of_subtree(root_node), 5);
    }
}

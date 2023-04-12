/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1305. All Elements in Two Binary Search Trees
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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
        if let Some(root_ref) = root {
            let root_node = root_ref.borrow();
            Self::dfs(&root_node.left, order);
            order.push(root_node.val);
            Self::dfs(&root_node.right, order);
        }
    }

    pub fn get_all_elements_dfs(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut root1_order: Vec<i32> = Vec::new();
        let mut root2_order: Vec<i32> = Vec::new();

        Self::dfs(&root1, &mut root1_order);
        Self::dfs(&root2, &mut root2_order);

        let mut order: Vec<i32> = Vec::new();

        let (mut i, mut j) = (0, 0);

        while i < root1_order.len() && j < root2_order.len() {
            if root1_order[i] < root2_order[j] {
                order.push(root1_order[i]);
                i += 1;
            } else {
                order.push(root2_order[j]);
                j += 1;
            }
        }

        while i < root1_order.len() {
            order.push(root1_order[i]);
            i += 1;
        }

        while j < root2_order.len() {
            order.push(root2_order[j]);
            j += 1;
        }

        order
    }

    pub fn get_all_elements_stack(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        let mut stack = vec![root1, root2];

        while let Some(root) = stack.pop() {
            if let Some(root_ref) = root {
                let mut root_node = root_ref.borrow_mut();
                result.push(root_node.val);
                stack.push(root_node.left.take());
                stack.push(root_node.right.take());
            }
        }

        result.sort();

        result
    }

    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut order: Vec<i32> = Vec::new();
        Self::dfs(&root1, &mut order);
        Self::dfs(&root2, &mut order);
        order.sort();
        order
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
        let input_data_1 = [Some(2), Some(1), Some(4)];
        let input_data_2 = [Some(1), Some(0), Some(3)];

        let root_1: Option<Rc<RefCell<TreeNode>>> = construct_tree(&input_data_1, 0);
        let root_2: Option<Rc<RefCell<TreeNode>>> = construct_tree(&input_data_2, 0);

        assert_eq!(
            Solution::get_all_elements(root_1, root_2),
            vec![0, 1, 1, 2, 3, 4]
        );
    }
}

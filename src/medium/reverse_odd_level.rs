/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 2415. Reverse Odd Levels of Binary Tree
Link: https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/
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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, level: i32) {
        if let Some(root_ref) = root {
            let mut root_node = root_ref.borrow_mut();

            if level & 1 == 1 {
                match (&root_node.left, &root_node.right) {
                    (Some(left_ref), Some(right_ref)) => {
                        let mut left_node = left_ref.borrow_mut();
                        let mut right_node = right_ref.borrow_mut();
                        // let temp = left_node.val;
                        // left_node.val = right_node.val
                        std::mem::swap(&mut left_node.val, &mut right_node.val);
                    }
                    _ => {}
                }
            }
            Self::dfs(&root_node.left, level + 1);
            Self::dfs(&root_node.right, level + 1);
        }
    }
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root, 0);
        root
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{Solution, TreeNode};

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) {
        match root {
            Some(root_ref) => {
                let root_node = root_ref.borrow();
                dfs(&root_node.left);
                print!("{} ", root_node.val);
                dfs(&root_node.right);
            }
            None => {}
        }
    }

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
        let input_data = [
            Some(0),
            Some(1),
            Some(2),
            Some(0),
            Some(0),
            Some(0),
            Some(0),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
        ];

        let root_node: Option<Rc<RefCell<TreeNode>>> = construct_tree(&input_data, 0);

        let result = Solution::reverse_odd_levels(root_node);

        dfs(&result);
        // assert_eq!(,);
    }
}

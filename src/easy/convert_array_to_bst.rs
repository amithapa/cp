/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 108. Convert Sorted Array to Binary Search Tree
Link: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::create_bst(&nums, 0, nums.len())
    }

    fn create_bst(nums: &[i32], start: usize, end: usize) -> Node {
        if start < end {
            let root_index = (start + end) / 2;

            let mut root_node = TreeNode::new(nums[root_index]);
            root_node.left = Self::create_bst(nums, start, root_index);
            root_node.right = Self::create_bst(nums, root_index + 1, end);

            Some(Rc::new(RefCell::new(root_node)))
        } else {
            None
        }
    }
}

fn dfs(root: &Node) {
    match root {
        Some(root_ref) => {
            let root_node = root_ref.borrow();
            print!("{} ", root_node.val);
            dfs(&root_node.left);
            dfs(&root_node.right);
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_i() {
        let input_data = vec![-10, -3, 0, 5, 9];
        let result = Solution::sorted_array_to_bst(input_data);

        super::dfs(&result);
    }
}

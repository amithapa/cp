/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1261. Find Elements in a Contaminated Binary Tree
Link: https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/
 */
/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
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
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Default)]
struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
    elements: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut elements = Self::default();
        elements.populate(&root, 0);
        elements
    }

    fn populate(&mut self, root: &Option<Rc<RefCell<TreeNode>>>, index: i32) {
        if let Some(root_ref) = root {
            let mut root_node = root_ref.borrow_mut();
            root_node.val = index;
            self.elements.insert(index);
            self.populate(&root_node.left, index * 2 + 1);
            self.populate(&root_node.right, index * 2 + 2);
        }
    }

    fn find(&self, target: i32) -> bool {
        self.elements.contains(&target)
    }
}

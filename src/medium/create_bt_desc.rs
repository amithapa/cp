/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 2196. Create Binary Tree From Descriptions
Link: https://leetcode.com/problems/create-binary-tree-from-descriptions/
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

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut tree = HashMap::new();

        let mut root_candidate = HashSet::new();
        let mut child_candidate = HashSet::new();

        for description in descriptions {
            let (parent, child, is_left) = (description[0], description[1], description[2]);

            let node = tree
                .entry(parent)
                .or_insert(Some(Rc::new(RefCell::new(TreeNode::new(parent)))))
                .clone()
                .unwrap();

            if !child_candidate.contains(&parent) {
                root_candidate.insert(parent);
            }
            child_candidate.insert(child);

            if root_candidate.contains(&child) {
                root_candidate.remove(&child);
            }

            // Some(Rc::new(RefCell::new(TreeNode::new(child))));
            let child_node = tree
                .entry(child)
                .or_insert(Some(Rc::new(RefCell::new(TreeNode::new(child)))))
                .clone();
            if is_left == 1 {
                node.borrow_mut().left = child_node;
            } else {
                node.borrow_mut().right = child_node;
            }
        }

        tree.remove(&root_candidate.iter().last().unwrap()).unwrap()
    }
}

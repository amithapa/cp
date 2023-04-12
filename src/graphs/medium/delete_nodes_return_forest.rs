/*
Source: LeetCode
Tags: Tree, DFS, Binary Tree
Problem: 1110. Delete Nodes And Return Forest
Link: https://leetcode.com/problems/delete-nodes-and-return-forest/
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
struct Solution;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn dfs(
        root: &Node,
        result: &mut Vec<Node>,
        to_delete: &mut HashSet<i32>,
        is_root: bool,
    ) -> Node {
        match root.clone() {
            Some(root_ref) => {
                let mut root_node = root_ref.borrow_mut();
                // checking if the root is to be deleted.
                let root_deleted = to_delete.contains(&root_node.val);

                // is a root node & the node is not marked for deletion hence pushing it to the result
                if is_root && !root_deleted {
                    result.push(root.clone());
                }

                // recursively compute on both left and right subtree.
                root_node.left = Self::dfs(&root_node.left.take(), result, to_delete, root_deleted);
                root_node.right =
                    Self::dfs(&root_node.right.take(), result, to_delete, root_deleted);

                // since it is marked for deletion we return None
                if root_deleted {
                    None
                } else {
                    root.clone()
                }
            }
            None => None,
        }
    }

    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut to_delete: HashSet<i32> = to_delete.into_iter().collect();
        let mut result: Vec<Node> = Vec::new();

        Self::dfs(&root, &mut result, &mut to_delete, true);

        result
    }
}

// Step4
// 目的: 指摘修正
// fn dfs() にちゃんと名前つける && 最小の深さを返すようにする

pub struct Solution;

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
impl Solution {
    fn is_leaf(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        left.is_none() && right.is_none()
    }
    // recursive dfs
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root_node) = root.as_ref() else {
            return 0;
        };

        Self::calculate_min_depth_path(root_node)
    }

    fn calculate_min_depth_path(node: &Rc<RefCell<TreeNode>>) -> i32 {
        let node_ref = node.borrow();

        if Self::is_leaf(&node_ref.left, &node_ref.right) {
            return 1;
        }

        let mut depth = i32::MAX;
        for child_node in [node_ref.left.as_ref(), node_ref.right.as_ref()]
            .into_iter()
            .flatten()
        {
            depth = std::cmp::min(depth, Self::calculate_min_depth_path(child_node));
        }
        depth + 1
    }
}

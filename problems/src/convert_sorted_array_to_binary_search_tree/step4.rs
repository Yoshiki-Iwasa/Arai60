// Step4
// 目的: 指摘対応
// 先にleft, rightをnodeに関連付けしておいてpopする時に値を入れる方式でやってみる

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

use core::num;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mut stack = VecDeque::new();
        let root = Rc::new(RefCell::new(TreeNode::new(i32::MIN)));

        // 開区間でやってみる
        stack.push_front((Rc::clone(&root), 0, nums.len()));

        while let Some((node, begin, end)) = stack.pop_front() {
            let mut node_mut_ref = node.borrow_mut();

            // right-middle
            let mid = (end - begin) / 2 + begin;
            node_mut_ref.val = nums[mid];
            if begin < mid {
                let left = Rc::new(RefCell::new(TreeNode::new(i32::MIN)));
                node_mut_ref.left = Some(Rc::clone(&left));
                stack.push_front((Rc::clone(&left), begin, mid))
            }
            if mid + 1 < end {
                let right = Rc::new(RefCell::new(TreeNode::new(i32::MIN)));
                node_mut_ref.right = Some(Rc::clone(&right));
                stack.push_front((Rc::clone(&right), mid + 1, end))
            }
        }

        Some(root)
    }
}

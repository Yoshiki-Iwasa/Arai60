// Step4
// 再帰かつ、indexで入れる方を分ける実装をやてみる

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

pub struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    fn zigzag_level_order_helper(
        node: Option<&Rc<RefCell<TreeNode>>>,
        level: usize,
        results: &mut Vec<VecDeque<i32>>,
    ) {
        let Some(node) = node else {
            return;
        };

        if level >= results.len() {
            results.push(VecDeque::new());
        }

        let current_level_nodes = &mut results[level];
        if level % 2 == 0 {
            current_level_nodes.push_back(node.borrow().val);
        } else {
            current_level_nodes.push_front(node.borrow().val);
        }
        Self::zigzag_level_order_helper(node.borrow().left.as_ref(), level + 1, results);
        Self::zigzag_level_order_helper(node.borrow().right.as_ref(), level + 1, results);
    }

    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut results = vec![];

        Self::zigzag_level_order_helper(root.as_ref(), 0, &mut results);

        results
            .into_iter()
            .map(|result| result.into())
            .collect::<Vec<_>>()
    }
}

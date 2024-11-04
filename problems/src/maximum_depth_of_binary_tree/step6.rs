// Step5
// 目的: 指摘対応

// while let Some()構文など、可読性をあげる
pub struct Solution;

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

type NodeDepthPair = (Rc<RefCell<TreeNode>>, u32);

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;

        let mut queue = VecDeque::<NodeDepthPair>::new();

        if let Some(root_node) = root {
            queue.push_back((Rc::clone(&root_node), 1));
        }

        while let Some((node, depth)) = queue.pop_front() {
            max_depth = std::cmp::max(max_depth, depth);

            for next_node in [node.borrow().left.as_ref(), node.borrow().right.as_ref()]
                .into_iter()
                .flatten()
            {
                queue.push_back((Rc::clone(next_node), depth + 1))
            }
        }

        max_depth as i32
    }
}

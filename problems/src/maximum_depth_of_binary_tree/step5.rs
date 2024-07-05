// Step5
// 目的: Step4から更に改善できそう
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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_depth_helper(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
            let Some(root_node) = root else {
                return depth;
            };

            let left = root_node.borrow().left.as_ref().map(Rc::clone);
            let right = root_node.borrow().right.as_ref().map(Rc::clone);

            std::cmp::max(
                max_depth_helper(left, depth + 1),
                max_depth_helper(right, depth + 1),
            )
        }

        max_depth_helper(root, 0)
    }
}

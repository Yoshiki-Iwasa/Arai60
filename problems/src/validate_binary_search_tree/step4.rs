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
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::in_order_validation(root.as_ref(), &mut None)
    }

    fn in_order_validation(
        node: Option<&Rc<RefCell<TreeNode>>>,
        prev_val: &mut Option<i32>,
    ) -> bool {
        let Some(node) = node else {
            return true;
        };
        let node_ref = node.borrow();
        if !Self::in_order_validation(node_ref.left.as_ref(), prev_val) {
            return false;
        }

        if prev_val.is_some_and(|prev_val| prev_val >= node_ref.val) {
            return false;
        }

        *prev_val = Some(node_ref.val);

        Self::in_order_validation(node_ref.right.as_ref(), prev_val)
    }
}

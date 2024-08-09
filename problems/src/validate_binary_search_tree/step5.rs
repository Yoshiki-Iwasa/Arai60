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
        type Node = Option<Rc<RefCell<TreeNode>>>;
        type LowerBound = i64;
        type UpperBound = i64;

        let mut stack = Vec::<(Node, LowerBound, UpperBound)>::new();

        stack.push((root, i64::MIN, i64::MAX));

        while let Some((node, lower_bound, upper_bound)) = stack.pop() {
            let Some(node) = node else {
                continue;
            };

            let node_ref = node.borrow();
            let node_val = node_ref.val as i64;

            if !(lower_bound < node_val && node_val < upper_bound) {
                return false;
            }

            stack.push((node_ref.left.as_ref().map(Rc::clone), lower_bound, node_val));
            stack.push((
                node_ref.right.as_ref().map(Rc::clone),
                node_val,
                upper_bound,
            ));
        }

        true
    }

    pub fn is_valid_bst_loop(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
        let mut lower_bounds = Vec::<i64>::new();
        let mut upper_bounds = Vec::<i64>::new();

        stack.push(root);
        lower_bounds.push(i64::MIN);
        upper_bounds.push(i64::MAX);

        while let (Some(node), Some(lower_bound), Some(upper_bound)) =
            (stack.pop(), lower_bounds.pop(), upper_bounds.pop())
        {
            let Some(node) = node else {
                continue;
            };

            let node_val = node.borrow().val as i64;

            if !(lower_bound < node_val && node_val < upper_bound) {
                return false;
            }

            stack.push(node.borrow().left.as_ref().map(Rc::clone));
            lower_bounds.push(lower_bound);
            upper_bounds.push(node_val);

            stack.push(node.borrow().right.as_ref().map(Rc::clone));
            lower_bounds.push(node_val);
            upper_bounds.push(upper_bound);
        }

        true
    }
}

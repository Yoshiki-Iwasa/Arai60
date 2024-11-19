// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(N)
  空間計算量: O(N)
*/

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

use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    fn is_leaf(node_ref: &Ref<TreeNode>) -> bool {
        node_ref.left.is_none() && node_ref.right.is_none()
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(root_node) = root else {
            return false;
        };

        let mut stack = VecDeque::new();
        stack.push_front((Rc::clone(&root_node), 0));

        while let Some((node, previous_sum)) = stack.pop_front() {
            let node_ref = node.borrow();
            let current_sum = node_ref.val + previous_sum;

            if Self::is_leaf(&node_ref) && current_sum == target_sum {
                return true;
            }

            for child_node in [node_ref.left.as_ref(), node_ref.right.as_ref()]
                .into_iter()
                .flatten()
            {
                stack.push_front((Rc::clone(child_node), current_sum))
            }
        }

        false
    }

    pub fn has_path_sum_recursive(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::has_path_sum_recursive_helper(root.as_ref(), target_sum)
    }

    fn has_path_sum_recursive_helper(node: Option<&Rc<RefCell<TreeNode>>>, mut rest: i32) -> bool {
        let Some(node) = node else {
            return false;
        };

        let node_ref = node.borrow();
        rest -= node_ref.val;
        if Self::is_leaf(&node_ref) && rest == 0 {
            return true;
        }

        Self::has_path_sum_recursive_helper(node_ref.left.as_ref(), rest)
            || Self::has_path_sum_recursive_helper(node_ref.right.as_ref(), rest)
    }
}

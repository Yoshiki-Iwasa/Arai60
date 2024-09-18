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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_helper(root.as_ref(), i64::MIN, i64::MAX)
    }

    fn is_valid_bst_helper(
        node: Option<&Rc<RefCell<TreeNode>>>,
        lower_bound: i64,
        upper_bound: i64,
    ) -> bool {
        let Some(node) = node else {
            return true;
        };

        let node_ref = node.borrow();
        let node_val = node_ref.val as i64;

        if !(lower_bound < node_val && node_val < upper_bound) {
            return false;
        }

        Self::is_valid_bst_helper(node_ref.left.as_ref(), lower_bound, node_val)
            && Self::is_valid_bst_helper(node_ref.right.as_ref(), node_val, upper_bound)
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

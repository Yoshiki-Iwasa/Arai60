// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(N^2): inorderを各ステップで走査するため
  空間計算量: O(N): 再帰の深さに依存するため
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
    fn split_exclusive_at(val: i32, slice: &[i32]) -> (&[i32], &[i32]) {
        for i in 0..slice.len() {
            if slice[i] == val {
                return (&slice[0..i], &slice[i + 1..]);
            }
        }
        (slice, &[])
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // Assume inputs are already validated
        Self::build_tree_helper(&preorder, &inorder)
    }

    pub fn build_tree_helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let (node_val, preorder_rest) = preorder.split_first()?;

        let (inorder_left, inorder_right) = Self::split_exclusive_at(*node_val, inorder);

        let preorder_left = &preorder_rest[..inorder_left.len()];
        let preorder_right = &preorder_rest[inorder_left.len()..];

        let mut node = TreeNode::new(*node_val);

        node.left = Self::build_tree_helper(preorder_left, inorder_left);
        node.right = Self::build_tree_helper(preorder_right, inorder_right);

        Some(Rc::new(RefCell::new(node)))
    }
}

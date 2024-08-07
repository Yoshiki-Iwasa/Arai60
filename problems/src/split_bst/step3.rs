// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  treeが一列になっていた場合
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

type SmallLargeTreePair = (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>);

impl Solution {
    pub fn split_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let (smaller, larger) = Self::split_bst_helper(root, target);
        vec![smaller, larger]
    }

    fn split_bst_helper(node: Option<Rc<RefCell<TreeNode>>>, target: i32) -> SmallLargeTreePair {
        let Some(node) = node else {
            return (None, None);
        };

        let mut node_mut_ref = node.borrow_mut();

        if node_mut_ref.val > target {
            // left treeに target以上の要素がある可能性あり
            let (smaller, mut larger) =
                Self::split_bst_helper(node_mut_ref.left.as_ref().map(Rc::clone), target);
            node_mut_ref.left = larger.take();
            (smaller, Some(Rc::clone(&node)))
        } else {
            // right treeにtarget以下の要素がある可能性あり
            let (mut smaller, larger) =
                Self::split_bst_helper(node_mut_ref.right.as_ref().map(Rc::clone), target);
            node_mut_ref.right = smaller.take();
            (Some(Rc::clone(&node)), larger)
        }
    }
}

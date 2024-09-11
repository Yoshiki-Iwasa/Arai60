// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  N は max(tree1.len(), tree2.len())
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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (None, Some(node2)) => Some(node2),
            (Some(node1), None) => Some(node1),
            (Some(node1), Some(node2)) => {
                let mut node1 = node1.borrow_mut();
                let mut node2 = node2.borrow_mut();

                let mut new_node = TreeNode::new(node1.val + node2.val);
                new_node.left = Self::merge_trees(node1.left.take(), node2.left.take());
                new_node.right = Self::merge_trees(node1.right.take(), node2.right.take());
                Some(Rc::new(RefCell::new(new_node)))
            }
        }
    }
}

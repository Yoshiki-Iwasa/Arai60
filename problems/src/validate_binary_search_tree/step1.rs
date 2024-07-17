// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  -

  何を考えて解いていたか
  - in-order traversalで掘っていけばいいのでは
    10^4程度ならstack overflowしないだろうからやってみる
    Option<&T>は 8byte (Optionはnull pointer optimizationあり)
        https://doc.rust-lang.org/stable/std/option/index.html#representation
    traverse_path: 8byte
    最悪ケース
    16 * 10 ^ 4 = 160,000
    Rustのstackは最大2MiB ≒ 208000,000なので余裕


  正解してから気づいたこと
  - 上界と下界を管理すればもっと楽に書けるのでは
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
        Self::in_order_validation(root.as_ref(), &mut vec![])
    }

    fn in_order_validation(
        node: Option<&Rc<RefCell<TreeNode>>>,
        traversal_path: &mut Vec<i32>,
    ) -> bool {
        let Some(node) = node else {
            return true;
        };
        let node_ref = node.borrow();
        if !Self::in_order_validation(node_ref.left.as_ref(), traversal_path) {
            return false;
        }

        if traversal_path
            .last()
            .is_some_and(|prev_val| *prev_val >= node_ref.val)
        {
            return false;
        }
        traversal_path.push(node_ref.val);

        Self::in_order_validation(node_ref.right.as_ref(), traversal_path)
    }
}

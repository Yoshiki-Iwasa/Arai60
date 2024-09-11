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
  - node の数が2000以内だから再帰的に書いても問題なさそう
    -10^4 <= node.val 10^4 なので、これも足し合わせて問題なし

    ノードを足してあげる操作を再帰的に繰り返せば良いのでは


  想定ユースケース
  -

  正解してから気づいたこと
  -
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

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
  - 行き止まりになるまで続ければいい
  - その上で最小値を更新しつづければいい
  - 再帰でも解けるが、最大で10^5の深さになる可能性があるのでloopでやったほうが良さそう
  - BFSで探索して左右どちらにもたどり着けないノードを見つけたら終わり
  - BFSから外にでることは無いからunreachableでいいのだけれど、なんかもっとうまい表現方法がないものだろうか...

  想定ユースケース
  - 行き止まりまでの距離をしりたいときってあるのか...?
    木のリーフまでの距離が知りたい場合ってどんなときだ...

  正解してから気づいたこと
  - queueにOptionのままで入れることはできない
    queue.push_back(opt)すると、optの所有権がqueueに移る
    optはもともとRc::borrow()で借用した変数のメンバにあたるのでmoveできない(参照の背後の変数はmoveできないルール)
*/

pub struct Solution;

// Definition for a binary tree node.
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

type NodeDepthPair = (Rc<RefCell<TreeNode>>, u32);

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    fn is_leaf(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        left.is_none() && right.is_none()
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root_node) = root else {
            return 0;
        };

        let mut queue = VecDeque::<NodeDepthPair>::new();

        queue.push_back((Rc::clone(&root_node), 1));

        while let Some((node, depth)) = queue.pop_front() {
            let node_ref = node.borrow();

            if Self::is_leaf(&node_ref.left, &node_ref.right) {
                return depth as i32;
            }

            if let Some(left_node) = node_ref.left.as_ref() {
                queue.push_back((Rc::clone(left_node), depth + 1));
            }

            if let Some(right_node) = node_ref.right.as_ref() {
                queue.push_back((Rc::clone(right_node), depth + 1));
            }
        }
        unreachable!()
    }
}

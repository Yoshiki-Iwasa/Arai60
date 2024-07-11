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
  - 深さ優先探索をしていけばいい
    loopで書いたほうが見通しが良さそう
    大きい入力にも対応できるように基本方針はloopを使う
    練習として再帰を書くが、それはstep2以降にする

  - 足し上げていく方法と引いていく方法がある
    今回は素直に足し上げていく方法にしてみる

  想定ユースケース
  - ちょうど運賃〇〇円で到達できる終点の駅は存在するか
    システム上で恒久的に動くより、スクリプトとかになりそう

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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    fn is_leaf(node: &Rc<RefCell<TreeNode>>) -> bool {
        node.borrow().left.is_none() && node.borrow().right.is_none()
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(root_node) = root else {
            return false;
        };

        let mut stack = VecDeque::new();

        stack.push_back((Rc::clone(&root_node), 0));

        while let Some((node, previous_sum)) = stack.pop_front() {
            let current_sum = node.borrow().val + previous_sum;
            if Self::is_leaf(&node) && current_sum == target_sum {
                return true;
            }

            for child_node in [node.borrow().left.as_ref(), node.borrow().right.as_ref()]
                .into_iter()
                .flatten()
            {
                stack.push_front((Rc::clone(child_node), current_sum))
            }
        }

        false
    }
}

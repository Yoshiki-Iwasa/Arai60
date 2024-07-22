// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - 冗長な処理を書き連ねてしまった
    いい感じの整理ができなかった

  何を考えて解いていたか
  - level毎に見ていくならBFSで良さそう
    この時、同じレベルのノードはqueueにすでに入ってるので、なくなるまで回す

  想定ユースケース
  - おもいつかず

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root_node) = root else {
            return vec![];
        };

        let mut queue = VecDeque::new();
        let mut node_groups = vec![];
        queue.push_back(Rc::clone(&root_node));

        while !queue.is_empty() {
            let mut nodes = vec![];
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();

                nodes.push(node_ref.val);
                for child in [node_ref.left.as_ref(), node_ref.right.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    queue.push_back(Rc::clone(child));
                }
            }
            node_groups.push(nodes)
        }
        node_groups
    }
}

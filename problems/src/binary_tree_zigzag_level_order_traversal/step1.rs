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
  - queueの進めかたはBFSでOK
    leftward, rightwardを切り替えて詰める
    入れる前にくるっと反転


  想定ユースケース
  -

  正解してから気づいたこと
  - current_level_nodesに入れる時 push_back と push_frontを切り替えればそれで済んだはず
    reversをn/2の要素に対して行うので、無駄なステップが発生している

    step2でうまいこと調整使用

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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };

        let mut queue = VecDeque::new();
        let mut zigzag_level_order_nodes = vec![];
        let mut is_leftward = false;

        queue.push_back(root);
        while !queue.is_empty() {
            let mut current_level_nodes = vec![];
            let mut next_level_nodes = vec![];
            while let Some(node) = queue.pop_front() {
                let node_ref = node.borrow();
                current_level_nodes.push(node_ref.val);

                // set rightward basically
                for child in [node_ref.left.as_ref(), node_ref.right.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    next_level_nodes.push(Rc::clone(child));
                }
            }

            if is_leftward {
                current_level_nodes.reverse()
            };
            zigzag_level_order_nodes.push(current_level_nodes);
            queue.extend(next_level_nodes);
            is_leftward = !is_leftward;
        }

        zigzag_level_order_nodes
    }
}

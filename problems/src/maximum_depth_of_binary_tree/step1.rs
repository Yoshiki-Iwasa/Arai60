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
  - 単純にDFSしながら最高値を更新していけばいいのでは
  - O(N)で終わる
  - 最悪10^4 の深さがあるので再帰はやめておく
  - Rc, RfCellなどRust特有の事情による複雑さがある
  - コーディングインタビューの時、この辺も聞かれそうだな

  想定ユースケース
  -

  正解してから気づいたこと
  -
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

use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::<(i32, i32)>::new();
        queue.push_back((Rc::clone(&root.unwrap()), 1));

        let mut max_depth = 0;

        while !queue.is_empty() {
            let (node, depth) = queue.pop_front().unwrap();

            max_depth = std::cmp::max(max_depth, depth);
            visited.insert((node.borrow().val, depth));
            for next_node in [node.borrow().left.as_ref(), node.borrow().right.as_ref()] {
                if let Some(next_node) = next_node {
                    if !visited.contains(&(next_node.borrow().val, depth + 1)) {
                        queue.push_back((Rc::clone(next_node), depth + 1))
                    }
                }
            }
        }

        max_depth
    }
}

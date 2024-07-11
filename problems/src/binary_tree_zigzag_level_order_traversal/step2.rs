// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時にかんがえたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  -

  他の人のコードを読んで考えたこと
  - https://github.com/fhiyo/leetcode/pull/29/files
    ほぼ同じこと考えて書いてそう
    step1で書いたリストを反転させる方法も可読性的には十分ありえるとのこと
    直感的な解法は効率的に最適でない可能性があるが、読みやすいはずなのでstep1では恐れずに書いてみよう

  - https://github.com/YukiMichishita/LeetCode/pull/11/files
    depthの偶奇でジグザグを分けてる実装
    depth毎にdequeを持って入れる方向を分けてる
    depth毎に持たなくても順次appendしたほうがわかりやすそう


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - current_level_nodesに詰める方向を切り替えるように書いてみる


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
            let mut current_level_nodes = VecDeque::new();
            let mut next_level_nodes = vec![];
            while let Some(node) = queue.pop_front() {
                let node_ref = node.borrow();
                match is_leftward {
                    true => current_level_nodes.push_front(node_ref.val),
                    false => current_level_nodes.push_back(node_ref.val),
                }

                // set rightward basically
                for child in [node_ref.left.as_ref(), node_ref.right.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    next_level_nodes.push(Rc::clone(child));
                }
            }

            zigzag_level_order_nodes.push(current_level_nodes.into());
            queue.extend(next_level_nodes);
            is_leftward = !is_leftward;
        }

        zigzag_level_order_nodes
    }
}

// VecDeque -> Vec は定数速度
// https://doc.rust-lang.org/src/alloc/collections/vec_deque/mod.rs.html#2877
// From traitが実装されればInto traitも自動実装される
// https://doc.rust-lang.org/src/core/convert/mod.rs.html#748

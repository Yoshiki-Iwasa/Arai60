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
  - https://github.com/fhiyo/leetcode/pull/28/files
    次のlevelのnodeたちを別の配列にためといてextendするという手もあるか
    current_level_nodes, next_level_nodesのほうがわかりやすいかも

  - https://github.com/sakupan102/arai60-practice/pull/27#pullrequestreview-2051073932
    実装とかより変数名がめんどくさいのかな
    基本的な実装の方針は同じなので、読みやすさの差異は


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 変数名にlevelごとの分類であることを反映する
  - unwrap()撲滅宣言
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
        let Some(root) = root else {
            return vec![];
        };

        let mut queue = VecDeque::new();
        let mut level_order_nodes = vec![];

        queue.push_back(Rc::clone(&root));

        while !queue.is_empty() {
            let mut current_level_nodes = vec![];
            let mut next_level_nodes = vec![];
            while let Some(node) = queue.pop_front() {
                let node_ref = node.borrow();

                current_level_nodes.push(node_ref.val);

                for child in [node_ref.left.as_ref(), node_ref.right.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    next_level_nodes.push(Rc::clone(child))
                }
            }
            queue.extend(next_level_nodes);
            level_order_nodes.push(current_level_nodes)
        }
        level_order_nodes
    }
}

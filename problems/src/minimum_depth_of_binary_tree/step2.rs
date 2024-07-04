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
  - unreachableを使わないで書けないか？
  - dfsでも書いてみよう

  他の人のコードを読んで考えたこと
  - https://github.com/fhiyo/leetcode/pull/24/files
    Rustで書く場合はif let Some() = ができるのでこれを使う
    この問題は解けるか解けないかより多くの反応ができるかどうかっぽい


  他の想定ユースケース
  -

  改善する時にかんがえたこと
  - left, rightでコピペになってるコードをまとめる
  - type aliasは使うところの近くに

  - うーん... unreachableをつかわないBFSが書きたいけど、うまくいかん...

  - DFSだと早く値を返すことができない
    一応すべて探索しないとそれが最小距離か判断がつかないから
    なら、BFSのほうがこの問題には向いていそう
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    fn is_leaf(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        left.is_none() && right.is_none()
    }

    // bfs
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root_node) = root else {
            return 0;
        };

        type NodeDepthPair = (Rc<RefCell<TreeNode>>, u32);

        let mut queue = VecDeque::<NodeDepthPair>::new();

        queue.push_back((root_node, 1));

        while let Some((node, depth)) = queue.pop_front() {
            let node_ref = node.borrow();

            if Self::is_leaf(&node_ref.left, &node_ref.right) {
                return depth as i32;
            }

            for child in [node_ref.left.as_ref(), node_ref.right.as_ref()] {
                if let Some(child_node) = child {
                    queue.push_back((Rc::clone(child_node), depth + 1));
                }
            }
        }
        unreachable!()
    }

    // dfs recursive
    pub fn min_depth_dfs_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root_node = root.unwrap();

        // u32::MAXにすると最後のキャストでオーバフローする可能性がある
        // 問題設定からその心配はないが、一応 i32::MAXで安全を取ることにする
        let mut min_depth = i32::MAX as u32;

        Self::dfs(root_node, 1, &mut min_depth);

        min_depth as i32
    }

    fn dfs(node: Rc<RefCell<TreeNode>>, depth: u32, min_depth: &mut u32) {
        let node_ref = node.borrow();

        if Self::is_leaf(&node_ref.left, &node_ref.right) {
            *min_depth = std::cmp::min(*min_depth, depth);
            return;
        }

        for child in [node_ref.left.as_ref(), node_ref.right.as_ref()] {
            if let Some(child_node) = child {
                Self::dfs(Rc::clone(child_node), depth + 1, min_depth)
            }
        }
    }
}

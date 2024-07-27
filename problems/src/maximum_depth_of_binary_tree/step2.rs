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
  - https://discord.com/channels/1084280443945353267/1235829049511903273/1252670332494680135
    制約的に厳しいとわかりつつ、練習で再帰を書いてるひともいるので自分も書く


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - depthを数えるなら自然数で行ったほうがいいので、u32でdepthをカウントする
  - tupleに名前をつける
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

type ValueDepthPair = (i32, u32);
type NodeDepthPair = (Rc<RefCell<TreeNode>>, u32);

use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut max_depth = 0_u32;

        let mut queue = VecDeque::<NodeDepthPair>::new();
        let mut visited = HashSet::<ValueDepthPair>::new();

        queue.push_back((Rc::clone(&root.unwrap()), 1));

        while !queue.is_empty() {
            let (node, depth) = queue.pop_front().unwrap();

            max_depth = std::cmp::max(max_depth, depth);
            visited.insert((node.borrow().val, depth));

            for next in [node.borrow().left.as_ref(), node.borrow().right.as_ref()] {
                if let Some(next_node) = next {
                    if !visited.contains(&(next_node.borrow().val, depth + 1)) {
                        queue.push_back((Rc::clone(&next_node), depth + 1));
                    }
                }
            }
        }

        max_depth as i32
    }

    pub fn max_depth_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_depth_helper(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
            if root.is_none() {
                return depth;
            }

            let root_node = root.unwrap();
            let left = if let Some(left_node) = root_node.borrow().left.as_ref() {
                Some(Rc::clone(&left_node))
            } else {
                None
            };

            let right = if let Some(right_node) = root_node.borrow().right.as_ref() {
                Some(Rc::clone(&right_node))
            } else {
                None
            };

            std::cmp::max(
                max_depth_helper(left, depth + 1),
                max_depth_helper(right, depth + 1),
            )
        }

        max_depth_helper(root, 0)
    }
}

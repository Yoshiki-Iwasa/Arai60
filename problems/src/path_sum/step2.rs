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
  - https://github.com/SuperHotDogCat/coding-interview/pull/37/files
    loopで書くのなら足し上げていく方が素直なのでは？
    再帰で書く場合は引いていった方が引数が少なくて良いと思う
  - https://github.com/fhiyo/leetcode/pull/27/files
    基本的に同じ方針ぽい

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - node.borrow()の繰り返しをやめる
  - is_leafの引数は`&Ref<TreeNode>`にする
  - 再帰関数を書くときは引き算する
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

use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    fn is_leaf(node_ref: &Ref<TreeNode>) -> bool {
        node_ref.left.is_none() && node_ref.right.is_none()
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(root_node) = root else {
            return false;
        };

        let mut stack = VecDeque::new();

        stack.push_back((Rc::clone(&root_node), 0));

        while let Some((node, previous_sum)) = stack.pop_front() {
            let node_ref = node.borrow();
            let current_sum = node_ref.val + previous_sum;
            if Self::is_leaf(&node_ref) && current_sum == target_sum {
                return true;
            }

            for child_node in [node_ref.left.as_ref(), node_ref.right.as_ref()]
                .into_iter()
                .flatten()
            {
                stack.push_front((Rc::clone(child_node), current_sum))
            }
        }

        false
    }

    // 再帰的に解く方法
    pub fn has_path_sum_recursive(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::has_path_sum_recursive_helper(root.as_ref(), target_sum)
    }

    // 再帰関数では参照を使用するため分けている
    // 参照を使用しない場合、Option<>の中身の参照を引き出して、Rc::cloneする必要がある
    // 無駄にコードを増やさないために参照を使う
    fn has_path_sum_recursive_helper(node: Option<&Rc<RefCell<TreeNode>>>, mut rest: i32) -> bool {
        let Some(node) = node else {
            return false;
        };
        let node_ref = node.borrow();
        rest -= node_ref.val;
        if Self::is_leaf(&node_ref) && rest == 0 {
            return true;
        }

        Self::has_path_sum_recursive_helper(node_ref.left.as_ref(), rest)
            || Self::has_path_sum_recursive_helper(node_ref.right.as_ref(), rest)
    }
}

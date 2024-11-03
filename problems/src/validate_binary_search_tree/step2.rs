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
  - step1でtraverse pathを持ってるのはメモリ無駄なのでは？


  他の人のコードを読んで考えたこと
  - https://github.com/kazukiii/leetcode/pull/29/files
    書く走査で上界と下界をもっているほうがシンプルに書けるんだな

  - https://github.com/fhiyo/leetcode/pull/30/files
    stackを使って書くのもありか



  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 上界と下界を管理してみた
    lowerとupperを数値でもつようにした
    Option<i32>も書いてみたがパターンマッチが冗長になったり、Rcの取り回しが複雑でメリットがあまりなかった
    loopより再帰のほうがコードは単純になっていいと思う


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
use std::rc::Rc;
impl Solution {
    // use lower and upper bound
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_helper(root.as_ref(), i64::MAX, i64::MIN)
    }

    fn is_valid_bst_helper(
        node: Option<&Rc<RefCell<TreeNode>>>,
        lower_bound: i64,
        upper_bound: i64,
    ) -> bool {
        let Some(node) = node else {
            return true;
        };
        let node_ref = node.borrow();

        // cast for comparison with lower and upper bound
        let node_val = node_ref.val as i64;

        if !(lower_bound < node_val && node_val > upper_bound) {
            return false;
        }

        Self::is_valid_bst_helper(node_ref.left.as_ref(), lower_bound, node_val)
            && Self::is_valid_bst_helper(node_ref.right.as_ref(), node_val, upper_bound)
    }

    // use lower and upper bound
    pub fn is_valid_bst_loop(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
        let mut lower_bounds = Vec::<i64>::new();
        let mut upper_bounds = Vec::<i64>::new();

        stack.push(root);
        lower_bounds.push(i64::MIN);
        upper_bounds.push(i64::MAX);

        while let (Some(node), Some(lower_bound), Some(upper_bound)) =
            (stack.pop(), lower_bounds.pop(), upper_bounds.pop())
        {
            let Some(node) = node else {
                continue;
            };

            let node_val = node.borrow().val as i64;

            if !(lower_bound < node_val && node_val < upper_bound) {
                return false;
            }

            stack.push(node.borrow().left.as_ref().map(Rc::clone));
            lower_bounds.push(lower_bound);
            upper_bounds.push(node_val);

            stack.push(node.borrow().right.as_ref().map(Rc::clone));
            lower_bounds.push(node_val);
            upper_bounds.push(upper_bound);
        }

        true
    }
}

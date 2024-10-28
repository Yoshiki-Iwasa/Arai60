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
  - https://github.com/Mike0121/LeetCode/pull/16/files
    分割後のtreeはleft, rightよりsmaller, largerのほうが性質の違いが表現できていていいと思った
    戻り値がleft, rightだとどっちから見てleft, rightなのかわからなくなりそうと思った

  - https://github.com/hayashi-ay/leetcode/pull/53
    基本的には同じ


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - node.borrow()を何回も書かなくて済むようにした
    RefCellは借用チェックを実行時に行うので、borrow_mut()する場合は関数の
    先頭でしてしまって、その後borrow()が起きないことを明確にしたほうがいいと思った

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

type SmallLargeTreePair = (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>);

impl Solution {
    pub fn split_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let (smaller, larger) = Self::split_bst_helper(root, target);
        vec![smaller, larger]
    }

    pub fn split_bst_helper(
        node: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> SmallLargeTreePair {
        let Some(node) = node else {
            return (None, None);
        };

        let mut node_mut_ref = node.borrow_mut();
        if node_mut_ref.val > target {
            let (smaller, mut larger) =
                Self::split_bst_helper(node_mut_ref.left.as_ref().map(Rc::clone), target);
            node_mut_ref.left = larger.take();
            (smaller, Some(Rc::clone(&node)))
        } else {
            let (mut smaller, larger) =
                Self::split_bst_helper(node_mut_ref.right.as_ref().map(Rc::clone), target);
            node_mut_ref.right = smaller.take();
            (Some(Rc::clone(&node)), larger)
        }
    }
}

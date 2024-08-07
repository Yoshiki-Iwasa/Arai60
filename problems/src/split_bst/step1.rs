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
  - パッと思いつくのは、探索していってtarget以下になったらそこでツリー
    を分割しちゃう案
    うまく再帰でやる方法は思いつかないので直感的なloopで書く

    これではうまくいかないことに気がつく
    解けなくてヒントをみてやり方に気がつく

    なるほど
    root.val > targetなら、left subtreeを分割してtargetより大きい方をroot.leftに、小さい方をsubtree rootにする
    root.val <= targetなら、right subtreeを分割してtargetより小さい方をroot.rightに、大きい方をsubtree rootにする

    なので、この分割プロセスを再帰的に繰り返せばいい。規定条件は


  想定ユースケース
  -

  正解してから気づいたこと
  - なぜこの問題が解けなかったのかを考える
    この問題をoriginalをsubtreeに分けると考えてしまったのがよくなかった
    targetとの大小比較で２つに分けると捉えられたら解けた気がする
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
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> SmallLargeTreePair {
        let Some(node) = root else {
            return (None, None);
        };

        // left treeをsplitするべき
        // right tree > targetが保証されている
        if node.borrow().val > target {
            let (smaller, mut larger) =
                Self::split_bst_helper(node.borrow().left.as_ref().map(Rc::clone), target);
            node.borrow_mut().left = larger.take();
            (smaller, Some(node))
        } else {
            // right treeをsplitするべき
            // left tree < targetが保証されている
            let (mut smaller, larger) =
                Self::split_bst_helper(node.borrow().right.as_ref().map(Rc::clone), target);
            node.borrow_mut().right = smaller.take();
            (Some(node), larger)
        }
    }
}

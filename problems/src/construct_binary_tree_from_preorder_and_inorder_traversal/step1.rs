// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - slice.split_first()という関数は知らなかった
    rust-analyzerの候補で出てきたから使えたが、本番の面接では出てこなかったと思う

  何を考えて解いていたか
  - preorder[0]がrootになる
    inorderをpreorder[0]で分割して、前半は左部分木、後半は右部分木とする
    preorderは左部部分木の長さだけとって、残りは右部分木とすればいい
    これを繰り返せばいい
    直感的には高々3000Nodeなので再帰でも大丈夫そう



  想定ユースケース
  -

  正解してから気づいたこと
  - 再帰で使用するstack sizeを計算してみる
    64bitマシンを想定
    引数: pointer(8byte) * 2 = 16 byte
    関数内:
        pointer(8byte) * 6 = 48byte
        TreeNode: 4 + 8 + 8 = 24byte
        ↑ Rc<T>はTと参照数を管理するRcBox<T>のポインタのみを保存する
          Option<>はポインタに対しnon null optimizationをするので8byteのまま
    正味: 88byte

    再帰全体の合計: 3000 * 88 = 264,000 byte
    stack size: 2MiB ≒ 2,080,000

    なのでOK



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
    fn split_exclusive_at(val: i32, slice: &[i32]) -> (&[i32], &[i32]) {
        for i in 0..slice.len() {
            if slice[i] == val {
                return (&slice[0..i], &slice[i + 1..]);
            }
        }
        (slice, &[])
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(&preorder, &inorder)
    }

    fn build_tree_helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let (root, rest_preorder) = preorder.split_first()?;

        let (inorder_left, inorder_right) = Self::split_exclusive_at(*root, inorder);

        let preorder_left = &rest_preorder[..inorder_left.len()];
        let preorder_right = &rest_preorder[inorder_left.len()..];

        let mut root_node = TreeNode::new(*root);

        root_node.left = Self::build_tree_helper(preorder_left, inorder_left);

        root_node.right = Self::build_tree_helper(preorder_right, inorder_right);

        Some(Rc::new(RefCell::new(root_node)))
    }
}

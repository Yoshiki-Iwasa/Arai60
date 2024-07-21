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
  - 不正な入力に対しての対応はどう考えている？
  -

  他の人のコードを読んで考えたこと
  - https://github.com/fhiyo/leetcode/pull/31/files
    stackに積む実装ではindexを管理するより配列そのものを分割して管理したほうがpreorder, inorderの性質を
    直接表現できるのではなかろうか

  - https://github.com/YukiMichishita/LeetCode/pull/12/files
    value_to_index_mapを作る解法のほうが個人的にはわかりにくい...
    動きとしては毎回rootから探索するのか




  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 再帰バージョンの中で、root->nodeにしてみる
    部分木の中ではrootなんだけど、全体ではただのnodeというコメントがあったため

  - loopの実装では、部分木のroot nodeを保持しないといけないので、扱う変数が複雑化しやすい
    preorder_restに違和感を感じてしまうので、stack pop時に値を更新するスタイルも書いてみた

  問題とは無関係だが`slice.first()`は
  ```
  pub const fn first_mut(&mut self) -> Option<&mut T> {
        if let [first, ..] = self { Some(first) } else { None }
    }
  ```
  となっていてそういうパターンマッチもあるんだと
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

    // 再帰的な解法
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // Assume inputs are already validated
        Self::build_tree_helper(&preorder, &inorder)
    }

    fn build_tree_helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let (node_val, rest_preorder) = preorder.split_first()?;

        let (inorder_left, inorder_right) = Self::split_exclusive_at(*node_val, inorder);

        let preorder_left = &rest_preorder[..inorder_left.len()];
        let preorder_right = &rest_preorder[inorder_left.len()..];

        let mut node = TreeNode::new(*node_val);

        node.left = Self::build_tree_helper(preorder_left, inorder_left);

        node.right = Self::build_tree_helper(preorder_right, inorder_right);

        Some(Rc::new(RefCell::new(node)))
    }

    // build_tree_helper()をloopで書いてみる
    #[allow(unused)]
    fn build_tree_helper_loop(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Assume inputs are already validated

        type PreorderSlice<'a> = &'a [i32];
        type InOrderSlice<'a> = &'a [i32];
        type Node = Rc<RefCell<TreeNode>>;

        let (root, rest) = preorder.split_first()?;

        let root = Rc::new(RefCell::new(TreeNode::new(*root)));

        let mut stack = Vec::<(Node, PreorderSlice, InOrderSlice)>::new();
        stack.push((Rc::clone(&root), rest, &inorder));

        while let Some((node, preorder_rest, inorder_sub)) = stack.pop() {
            if preorder_rest.is_empty() {
                continue;
            }
            let (inorder_left, inorder_right) =
                Self::split_exclusive_at(node.borrow().val, inorder_sub);

            let preorder_left = &preorder_rest[..inorder_left.len()];
            let preorder_right = &preorder_rest[inorder_left.len()..];

            let mut node_ref_mut = node.borrow_mut();
            if let Some(left) = preorder_left.first() {
                let left_node = Rc::new(RefCell::new(TreeNode::new(*left)));
                node_ref_mut.left = Some(Rc::clone(&left_node));
                stack.push((Rc::clone(&left_node), &preorder_left[1..], inorder_left))
            }

            if let Some(right) = preorder_right.first() {
                let right_node = Rc::new(RefCell::new(TreeNode::new(*right)));
                node_ref_mut.right = Some(Rc::clone(&right_node));
                stack.push((Rc::clone(&right_node), &preorder_right[1..], inorder_right))
            }
        }

        Some(root)
    }

    // stackにnodeをpushするとき、dummyの値をいれてpop時に更新するバージョン
    #[allow(unused)]
    fn build_tree_helper_loop_2(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Assume inputs are already validated

        type PreorderSlice<'a> = &'a [i32];
        type InOrderSlice<'a> = &'a [i32];
        type Node = Rc<RefCell<TreeNode>>;

        let root = Rc::new(RefCell::new(TreeNode::new(i32::MAX)));

        let mut stack = Vec::<(Node, PreorderSlice, InOrderSlice)>::new();
        stack.push((Rc::clone(&root), &preorder, &inorder));

        while let Some((node, preorder, inorder_sub)) = stack.pop() {
            // &[] にはpop()が無いので、逐次確認
            if let Some(node_val) = preorder.first() {
                node.borrow_mut().val = *node_val;
            } else {
                continue;
            };

            let preorder_rest = &preorder[1..];

            let (inorder_left, inorder_right) =
                Self::split_exclusive_at(node.borrow().val, inorder_sub);

            let preorder_left = &preorder_rest[..inorder_left.len()];
            let preorder_right = &preorder_rest[inorder_left.len()..];

            let mut node_ref_mut = node.borrow_mut();
            if !preorder_left.is_empty() {
                let left_node = Rc::new(RefCell::new(TreeNode::new(i32::MAX)));
                node_ref_mut.left = Some(Rc::clone(&left_node));
                stack.push((Rc::clone(&left_node), preorder_left, inorder_left))
            }

            if !preorder_right.is_empty() {
                let right_node = Rc::new(RefCell::new(TreeNode::new(i32::MAX)));
                node_ref_mut.right = Some(Rc::clone(&right_node));
                stack.push((Rc::clone(&right_node), preorder_right, inorder_right))
            }
        }

        Some(root)
    }
}

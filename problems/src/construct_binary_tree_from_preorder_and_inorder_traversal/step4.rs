// Step4
// inorder, preorderで頭から構築する
// https://github.com/kazukiii/leetcode/pull/30#discussion_r1689821100

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
    // preorderの順で構築
    // inorderをつかって左部分木にいるか右部分木にいるかを判断する
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let (root, rest_preorder) = preorder.split_first()?;
        let root = Rc::new(RefCell::new(TreeNode::new(*root)));
        let mut stack = Vec::new();
        // preorderに基づいて、親候補となるノードをstackに追加
        stack.push(Rc::clone(&root));

        let mut inorder_index = 0;
        // preorderの順（根->左部分木->右部分木）でノードを作成
        rest_preorder.iter().for_each(|val| {
            let node = Rc::new(RefCell::new(TreeNode::new(*val)));

            if let Some(mut parent_candidate) = stack.last().map(Rc::clone) {
                let parent_val = parent_candidate.borrow().val;

                match parent_val == inorder[inorder_index] {
                    true => {
                        // 親候補の値とinorderの要素が一致するとき
                        // parent_candidateを根としたとき、inorderにおける(左部分木 -> 根) が終わったということ
                        // 現在のnodeはparent_candidateの以上で右部分木をもつノードの最初の右子になる
                        while stack
                            .last()
                            .is_some_and(|top| top.borrow().val == inorder[inorder_index])
                        {
                            parent_candidate = stack.pop().unwrap();
                            inorder_index += 1;
                        }
                        parent_candidate.borrow_mut().right = Some(Rc::clone(&node))
                    }
                    false => {
                        // 親候補の値とinorderの要素が一致しない
                        // つまり、親候補を根としたとき、inorder traversalにおける(左部分木 -> 根) が終わってない
                        // 左部分木の構築がまだ終わってないので、nodeは左に入る
                        parent_candidate.borrow_mut().left = Some(Rc::clone(&node))
                    }
                }
            }

            stack.push(Rc::clone(&node))
        });

        Some(root)
    }
}

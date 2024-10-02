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
  - https://github.com/goto-untrapped/Arai60/pull/47/files#r1733144729
    破壊、非破壊どっちでも書ける
    非破壊にするなら、node1.borrow().right.as_ref.map(Rc::clone)みたいにちょっと長くなる
  - https://github.com/Ryotaro25/leetcode_first60/pull/25/files
  - https://github.com/TORUS0818/leetcode/pull/25/files



  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - とりあえず、stackに積んで書いてみる
  - 非破壊バージョンも書いてみる
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
    // loopで書いてみる
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = vec![];
        if root1.is_none() && root2.is_none() {
            return None;
        }

        let new_root = Rc::new(RefCell::new(TreeNode::new(i32::MAX)));

        stack.push((root1, root2, Rc::clone(&new_root)));

        while let Some((root1, root2, new_node)) = stack.pop() {
            let mut new_node = new_node.borrow_mut();
            match (root1, root2) {
                (Some(node1), Some(node2)) => {
                    let node1 = node1.borrow();
                    let node2 = node2.borrow();

                    new_node.val = node1.val + node2.val;

                    if !(node1.left.is_none() && node2.left.is_none()) {
                        let new_left = Rc::new(RefCell::new(TreeNode::new(i32::MAX)));
                        new_node.left = Some(Rc::clone(&new_left));
                        stack.push((
                            node1.left.as_ref().map(Rc::clone),
                            node2.left.as_ref().map(Rc::clone),
                            new_left,
                        ));
                    }

                    if !(node1.right.is_none() && node2.right.is_none()) {
                        let new_right = Rc::new(RefCell::new(TreeNode::new(i32::MAX)));
                        new_node.right = Some(Rc::clone(&new_right));
                        stack.push((
                            node1.right.as_ref().map(Rc::clone),
                            node2.right.as_ref().map(Rc::clone),
                            new_right,
                        ))
                    }
                }
                (None, None) => unreachable!(),
                (None, Some(node2)) => {
                    let node2 = node2.borrow();
                    new_node.val = node2.val;
                    new_node.left = node2.left.as_ref().map(Rc::clone);
                    new_node.right = node2.right.as_ref().map(Rc::clone);
                }
                (Some(node1), None) => {
                    let node1 = node1.borrow();
                    new_node.val = node1.val;
                    new_node.left = node1.left.as_ref().map(Rc::clone);
                    new_node.right = node1.right.as_ref().map(Rc::clone);
                }
            }
        }

        Some(new_root)
    }

    // 再帰非破壊
    pub fn merge_trees_2(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (None, Some(node2)) => Some(node2),
            (Some(node1), None) => Some(node1),
            (Some(node1), Some(node2)) => {
                let node1 = node1.borrow();
                let node2 = node2.borrow();

                let mut new_node = TreeNode::new(node1.val + node2.val);
                new_node.left = Self::merge_trees(
                    node1.left.as_ref().map(Rc::clone),
                    node2.left.as_ref().map(Rc::clone),
                );
                new_node.right = Self::merge_trees(
                    node1.right.as_ref().map(Rc::clone),
                    node2.right.as_ref().map(Rc::clone),
                );
                Some(Rc::new(RefCell::new(new_node)))
            }
        }
    }
}

// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(N)
  空間計算量: O(N)
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return vec![];
        };

        let mut queue = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        let mut level_order_nodes = vec![];

        queue.push_back(Rc::clone(&root));

        while !queue.is_empty() {
            let mut current_level_nodes = vec![];
            let mut next_level_nodes = vec![];

            while let Some(node) = queue.pop_front() {
                let node_ref = node.borrow();
                current_level_nodes.push(node_ref.val);

                for child in [node_ref.left.as_ref(), node_ref.right.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    next_level_nodes.push(Rc::clone(child));
                }
            }
            level_order_nodes.push(current_level_nodes);
            queue.extend(next_level_nodes);
        }
        level_order_nodes
    }
}

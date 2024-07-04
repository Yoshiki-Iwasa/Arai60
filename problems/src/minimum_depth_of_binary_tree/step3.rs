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

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    fn is_leaf(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        left.is_none() && right.is_none()
    }
    // bfs
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root_node) = root else {
            return 0;
        };

        type NodeDepthPair = (Rc<RefCell<TreeNode>>, u32);
        let mut queue = VecDeque::<NodeDepthPair>::new();

        queue.push_back((root_node, 1));

        while let Some((node, depth)) = queue.pop_front() {
            let node_ref = node.borrow();

            if Self::is_leaf(&node_ref.left, &node_ref.right) {
                return depth as i32;
            }

            for child in [node_ref.left.as_ref(), node_ref.right.as_ref()] {
                if let Some(child_node) = child {
                    queue.push_back((Rc::clone(child_node), depth + 1));
                }
            }
        }

        unreachable!()
    }

    // dfs recursive
    pub fn min_depth_dfs_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root_node = root.unwrap();

        // u32::MAXにすると最後のキャストでオーバフローする可能性がある
        // 問題設定からその心配はないが、一応 i32::MAXで安全を取ることにする
        let mut min_depth = i32::MAX as u32;

        Self::dfs(root_node, 1, &mut min_depth);

        min_depth as i32
    }

    fn dfs(node: Rc<RefCell<TreeNode>>, depth: u32, min_depth: &mut u32) {
        let node_ref = node.borrow();

        if Self::is_leaf(&node_ref.left, &node_ref.right) {
            *min_depth = std::cmp::min(*min_depth, depth);
            return;
        }

        for child in [node_ref.left.as_ref(), node_ref.right.as_ref()] {
            if let Some(child_node) = child {
                Self::dfs(Rc::clone(child_node), depth + 1, min_depth)
            }
        }
    }
}

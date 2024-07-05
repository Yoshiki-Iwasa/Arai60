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

type ValueDepthPair = (i32, u32);
type NodeDepthPair = (Rc<RefCell<TreeNode>>, u32);

use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut max_depth = 0;
        let mut queue = VecDeque::<NodeDepthPair>::new();
        let mut visited = HashSet::<ValueDepthPair>::new();

        queue.push_back((Rc::clone(root.as_ref().unwrap()), 1));

        while !queue.is_empty() {
            let (node, depth) = queue.pop_front().unwrap();

            max_depth = std::cmp::max(max_depth, depth);
            visited.insert((node.borrow().val, depth));

            for next in [node.borrow().left.as_ref(), node.borrow().right.as_ref()] {
                if let Some(next_node) = next {
                    if !visited.contains(&(next_node.borrow().val, depth + 1)) {
                        queue.push_back((Rc::clone(next_node), depth + 1))
                    }
                }
            }
        }

        max_depth as i32
    }

    pub fn max_depth_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_depth_helper(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
            if root.is_none() {
                return depth;
            }

            let root_node = root.unwrap();
            let left = if let Some(left_node) = root_node.borrow().left.as_ref() {
                Some(Rc::clone(&left_node))
            } else {
                None
            };

            let right = if let Some(right_node) = root_node.borrow().right.as_ref() {
                Some(Rc::clone(&right_node))
            } else {
                None
            };

            std::cmp::max(
                max_depth_helper(left, depth + 1),
                max_depth_helper(right, depth + 1),
            )
        }

        max_depth_helper(root, 0)
    }
}
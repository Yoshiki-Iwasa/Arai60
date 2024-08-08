// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量:
  空間計算量:
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

use core::num;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 余計なコピーをさけるためにヘルパー関数は参照を受ける
        fn sorted_array_to_bst_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }

            // right-middle
            let mid = nums.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: sorted_array_to_bst_helper(&nums[0..mid]),
                right: sorted_array_to_bst_helper(&nums[mid + 1..]),
            })))
        }
        sorted_array_to_bst_helper(&nums)
    }

    pub fn sorted_array_to_bst_loop_right_middle(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        enum Target {
            Left,
            Right,
        }
        if nums.is_empty() {
            return None;
        }

        // right-middle
        let top_index = nums.len() / 2;
        let top_node = Rc::new(RefCell::new(TreeNode {
            val: nums[top_index],
            left: None,
            right: None,
        }));

        let mut stack = VecDeque::new();

        stack.push_front((Rc::clone(&top_node), &nums[0..top_index], Target::Left));
        stack.push_front((Rc::clone(&top_node), &nums[top_index + 1..], Target::Right));

        while let Some((node, sub_nums, target)) = stack.pop_front() {
            if sub_nums.is_empty() {
                continue;
            }

            let mid = sub_nums.len() / 2;
            let new_node = Rc::new(RefCell::new(TreeNode {
                val: sub_nums[mid],
                left: None,
                right: None,
            }));

            match target {
                Target::Left => node.borrow_mut().left = Some(Rc::clone(&new_node)),
                Target::Right => node.borrow_mut().right = Some(Rc::clone(&new_node)),
            };

            stack.push_front((Rc::clone(&new_node), &sub_nums[0..mid], Target::Left));
            stack.push_front((Rc::clone(&new_node), &sub_nums[mid + 1..], Target::Right));
        }

        Some(top_node)
    }
}

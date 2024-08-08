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
  - https://github.com/fhiyo/leetcode/pull/26/files
    begin, endを比べる方法は最初は思いつかなかった
    二分探索ならこっちのほうが一般的なのだろう

    stackをもちいたloopによる実装も書いてみる

    bstを作る時、midの選び方はleft-middle, right-middleの二種類あるのでどちらも書いてみる

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 再帰の実装をもう少し短くする
  - loopバージョンで、left-middle, right-middle両方書いてみる
  - start, endを取る方法だと扱う変数が多くなって読みにくいのでnumsのsliceを引き回すことにする
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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 余計なコピーをさけるためにヘルパー関数は参照を受ける
        fn sorted_array_to_bst_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }

            // right_middle
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
        };

        let nums = nums.as_slice();

        let mut stack = VecDeque::new();

        // right-middle
        let top_index = nums.len() / 2;
        let top_node = Rc::new(RefCell::new(TreeNode {
            val: nums[top_index],
            left: None,
            right: None,
        }));

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
            stack.push_front((Rc::clone(&new_node), &sub_nums[mid + 1..], Target::Right))
        }

        Some(top_node)
    }

    fn get_left_middle(nums_len: usize) -> usize {
        if nums_len % 2 == 0 {
            nums_len / 2 - 1
        } else {
            nums_len / 2
        }
    }

    pub fn sorted_array_to_bst_loop_left_middle(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        enum Target {
            Left,
            Right,
        }

        if nums.is_empty() {
            return None;
        };

        let nums = nums.as_slice();

        let mut stack = VecDeque::new();

        // left-middle
        let top_index = Self::get_left_middle(nums.len());
        let top_node = Rc::new(RefCell::new(TreeNode {
            val: nums[top_index],
            left: None,
            right: None,
        }));

        stack.push_front((Rc::clone(&top_node), &nums[0..top_index], Target::Left));
        stack.push_front((Rc::clone(&top_node), &nums[top_index + 1..], Target::Right));

        while let Some((node, sub_nums, target)) = stack.pop_front() {
            if sub_nums.is_empty() {
                continue;
            }

            let mid = Self::get_left_middle(sub_nums.len());
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
            stack.push_front((Rc::clone(&new_node), &sub_nums[mid + 1..], Target::Right))
        }

        Some(top_node)
    }
}

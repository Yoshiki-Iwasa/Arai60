// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - loopでもきれいにやる方法

  何を考えて解いていたか
  - binary search treeということで、大きいのが右、小さいのが左になるように配置してく
  - 二分探索をやっていって、ノードをつけていってあげればいいか

  想定ユースケース
  -

  正解してから気づいたこと
  -
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
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // 余計なコピーをさけるためにヘルパー関数は参照を受ける
        fn sorted_array_to_bst_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }

            let top_index = nums.len() / 2;
            let top_node = Rc::new(RefCell::new(TreeNode {
                val: nums[top_index],
                left: None,
                right: None,
            }));

            top_node.borrow_mut().left = sorted_array_to_bst_helper(&nums[0..top_index]);
            top_node.borrow_mut().right = sorted_array_to_bst_helper(&nums[top_index + 1..]);

            Some(top_node)
        }
        sorted_array_to_bst_helper(&nums)
    }
}

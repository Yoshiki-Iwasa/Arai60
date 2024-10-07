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
  空間計算量: O(1)
*/
pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        std::cmp::max(
            Self::rob_helper(&nums[1..]),
            Self::rob_helper(&nums[0..nums.len() - 1]),
        )
    }

    fn rob_helper(nums: &[i32]) -> i32 {
        let Some(last) = nums.last() else {
            return 0; // nums is empty
        };

        let mut current_sum = *last;
        let mut post_sum = 0; // steal from n-th + 1 house

        (0..nums.len() - 1).rev().for_each(|i| {
            let max_sum = std::cmp::max(current_sum, post_sum + nums[i]);
            post_sum = current_sum;
            current_sum = max_sum
        });
        current_sum
    }
}

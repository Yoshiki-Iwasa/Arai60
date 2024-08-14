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

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut max_amounts_onward = vec![0; nums.len() + 1];

        max_amounts_onward[nums.len()] = 0;
        max_amounts_onward[nums.len() - 1] = nums[nums.len() - 1];

        (0..nums.len() - 1).rev().for_each(|i| {
            max_amounts_onward[i] = std::cmp::max(
                max_amounts_onward[i + 1],
                max_amounts_onward[i + 2] + nums[i],
            );
        });
        max_amounts_onward[0]
    }

    pub fn rob_2(nums: Vec<i32>) -> i32 {
        let Some(last) = nums.last() else {
            // nums is empty
            return 0;
        };

        let mut post_sum = 0;
        let mut current_sum = *last;

        (0..nums.len() - 1).rev().for_each(|i| {
            let max_sum = std::cmp::max(current_sum, post_sum + nums[i]);
            post_sum = current_sum;
            current_sum = max_sum;
        });
        current_sum
    }
}

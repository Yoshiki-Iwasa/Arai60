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
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let Some(first) = nums.first() else {
            return 0;
        };
        let mut current_max_sum = *first;
        let mut max_subarray_sum = *first;

        (1..nums.len()).for_each(|i| {
            current_max_sum = std::cmp::max(current_max_sum + nums[i], nums[i]);
            max_subarray_sum = max_subarray_sum.max(current_max_sum);
        });

        max_subarray_sum
    }
}

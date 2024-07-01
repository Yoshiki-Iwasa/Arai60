// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(n)
  空間計算量: O(n)
*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cumulative_sum_count_map = HashMap::<i32, u32>::new();

        let mut sum = 0;
        let mut subarray_count = 0;

        // cumulative_sum
        cumulative_sum_count_map.insert(0, 1);

        for n in nums {
            sum += n;
            let compliment = sum - k;
            if let Some(cnt) = cumulative_sum_count_map.get(&compliment) {
                subarray_count += cnt
            }
            *cumulative_sum_count_map.entry(sum).or_insert(0) += 1;
        }

        subarray_count as i32
    }
}

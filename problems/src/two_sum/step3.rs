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

pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut compliment_map = HashMap::<i32, usize>::new();

        for (index, n) in nums.iter().enumerate() {
            let compliment = target - *n;
            match compliment_map.get(&compliment) {
                Some(compliment_index) => return vec![index as i32, *compliment_index as i32],
                None => {
                    compliment_map.insert(*n, index);
                }
            }
        }
        eprintln!(
            "No pairs can make the target. nums: {:?}, target: {:?}",
            nums, target
        );
        vec![]
    }
}

// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

// 自分が苦手なbit全探索を書いておく

/*
  時間計算量: O(2^N * N!)
  空間計算量: O(2^N)
*/

pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        (0_u32..(1 << nums.len()))
            .map(|bit_pattern| {
                // bit pattern
                (0..nums.len())
                    .flat_map(|i| match (bit_pattern & (1 << i)) != 0 {
                        true => Some(nums[i]),
                        false => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

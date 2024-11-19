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
    pub fn next_permutation(nums: &mut [i32]) {
        let Some(pivot_index) = nums.windows(2).rposition(|w| w[0] < w[1]) else {
            nums.reverse();
            return;
        };

        let Some(rightmost_successor_index) = nums.iter().rposition(|&n| n > nums[pivot_index])
        else {
            unreachable!();
        };

        nums.swap(pivot_index, rightmost_successor_index);
        nums[pivot_index + 1..].reverse()
    }
}

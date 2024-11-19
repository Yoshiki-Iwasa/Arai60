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
    pub fn move_zeroes(nums: &mut [i32]) {
        // last_nozero_index 未満はすべてnozero
        let mut last_nozero_index = 0;

        (0..nums.len()).for_each(|index| {
            if nums[index] != 0 {
                nums.swap(index, last_nozero_index);
                last_nozero_index += 1;
            }
        })
    }
}

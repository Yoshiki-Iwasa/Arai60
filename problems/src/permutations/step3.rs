// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

// 樹形図をバックトラックしていくのがいちばんわかりやすかった

/*
  時間計算量: O(n!)
  空間計算量: O(n*n!)
*/

pub struct Solution;

impl Solution {
    // swappingで生成する
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::swapping_permutation(&mut nums, 0, &mut result);
        result
    }

    pub fn swapping_permutation(
        nums: &mut [i32],
        swap_index: usize,
        permutation: &mut Vec<Vec<i32>>,
    ) {
        if swap_index == nums.len() {
            permutation.push(nums.to_vec());
            return;
        }

        (swap_index..nums.len()).for_each(|i| {
            nums.swap(swap_index, i);
            Self::swapping_permutation(nums, swap_index + 1, permutation);
            nums.swap(swap_index, i);
        })
    }
}

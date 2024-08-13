// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(N logN)
  空間計算量: O(N)
*/

pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut lis = vec![];

        nums.into_iter().for_each(|n| {
            let insert_pos = lis.partition_point(|num_in_lis| *num_in_lis < n);
            match insert_pos >= lis.len() {
                true => lis.push(n),
                false => lis[insert_pos] = n,
            }
        });
        lis.len() as i32
    }
}

// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(N * M)
  空間計算量: O(N * M)
*/

pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;

        let mut num_of_paths = vec![vec![0; m]; n];

        (0..n).for_each(|i| num_of_paths[i][0] = 1);
        (0..m).for_each(|i| num_of_paths[0][i] = 1);

        for i in 1..n {
            for j in 1..m {
                num_of_paths[i][j] = num_of_paths[i - 1][j] + num_of_paths[i][j - 1];
            }
        }
        num_of_paths[n - 1][m - 1]
    }
}

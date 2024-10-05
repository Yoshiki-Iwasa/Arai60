// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時にかんがえたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  -

  他の人のコードを読んで考えたこと
  - https://github.com/goto-untrapped/Arai60/pull/32/files
    combinationで解いてる。なぜ
    https://github.com/hayashi-ay/leetcode/pull/39/files
    なるほど、水平方向と垂直方向の組み合わせの中からどれを選ぶかか


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - Top rowとleft colの初期化は明示的に書いたほうが意図が伝わりやすそう
*/

pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        // num_of_paths[i][j] is the number of paths to achieve i,j point (y = i, x = j) of the grid
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

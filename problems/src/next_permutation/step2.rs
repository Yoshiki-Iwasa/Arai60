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
  - https://github.com/fhiyo/leetcode/pull/56/files#diff-b3acad5bdbe668fe91a5e77ce0767a93ade655219d05322a3666b9f6b10907eeR107
    二分探索でも探せるということだけは頭に入れておこう

  - https://github.com/Mike0121/LeetCode/pull/15/files
    swapする位置を探す関数の切り出し方がよく議論されている
    Rustの場合は標準ライブラリの関数が充実してるので関数に切り出すまでもなさそう

  - https://github.com/Mike0121/LeetCode/pull/15/files#diff-18fcbdba94ed89573b586772a3747e1e4f3a5625a98e6782d2bfca4a3fd46027R90
    二重ループによる解法
    やってることは同じだけど、書き方が違う


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 二重ループによる解法も書いてみる
*/
pub struct Solution;

impl Solution {
    pub fn next_permutation_1(nums: &mut [i32]) {
        let Some(pivot_index) = nums.windows(2).rposition(|w| w[0] < w[1]) else {
            nums.reverse();
            return;
        };

        let Some(rightmost_successor_index) = nums.iter().rposition(|&n| n > nums[pivot_index])
        else {
            unreachable!()
        };

        nums.swap(pivot_index, rightmost_successor_index);
        nums[pivot_index + 1..].reverse();
    }

    // 二重ループの場合
    // 本当は、nums.len()-2やnums.len()-1で閉じてもいいが、変にミスりたくないのでnums.len()で閉じる
    pub fn next_permutation_2(nums: &mut [i32]) {
        for pivot_index in (0..nums.len()).rev() {
            for swap_index in (pivot_index..nums.len()).rev() {
                if nums[pivot_index] < nums[swap_index] {
                    nums.swap(pivot_index, swap_index);
                    nums[pivot_index + 1..].reverse();
                    return;
                }
            }
        }
        nums.reverse()
    }
}

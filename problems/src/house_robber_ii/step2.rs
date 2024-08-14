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
  - https://github.com/goto-untrapped/Arai60/pull/37/files#
  - https://github.com/fhiyo/leetcode/pull/37/files

    max amountを考えるとき、お尻から最大値を更新するような実装にすれば場合分けが減りそう

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - Step1のmax_amountsのお尻２つしか使わないので
    rob_helperは空間をケチるように書いた


*/

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        std::cmp::max(
            Self::rob_helper(&nums[1..]),
            Self::rob_helper(&nums[0..nums.len() - 1]),
        )
    }

    fn rob_helper(nums: &[i32]) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut post_sum = 0;
        let mut current_sum = nums[nums.len() - 1];

        (0..nums.len() - 1).rev().for_each(|i| {
            let max_sum = std::cmp::max(current_sum, post_sum + nums[i]);
            post_sum = current_sum;
            current_sum = max_sum;
        });
        current_sum
    }
}

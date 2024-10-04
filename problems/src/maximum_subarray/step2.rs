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
  - https://github.com/goto-untrapped/Arai60/commit/88cd60fdfcfc076ea65ddfdb68dcecb9fee18212
    やはり自然に思いつくのは累積和を使った全探索なのか

  - https://discord.com/channels/1084280443945353267/1206101582861697046/1207405733667410051
    このアルゴリズムを所与のものとして覚えるとかは違うんだろうな
    問題の条件から思いつきたかった

  Kadane's algorithm を使わないとTLEっぽい
  それってどうなんとは思う。
  おそらくインタビューではBrute forceの解き方の方を書いちゃってたと思う


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 配列が空のとき0を返すようにする(なにも選ばない)
  - Brute forceの方は累積和を準備しないでやる方法にしてみた
    どちらにしろTLEだけど
*/

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let Some(first) = nums.first() else {
            return 0;
        };

        let mut current_max_sum = *first;
        let mut max_subarray_sum = *first;

        (1..nums.len()).for_each(|i| {
            current_max_sum = std::cmp::max(current_max_sum + nums[i], nums[i]);
            max_subarray_sum = std::cmp::max(max_subarray_sum, current_max_sum);
        });

        max_subarray_sum
    }

    pub fn max_sub_array_tle(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        };
        let mut max_subarray_sum = i32::MIN;
        (0..nums.len()).for_each(|i| {
            let mut current_max_sum = 0;
            (i..nums.len()).for_each(|j| {
                current_max_sum += nums[j];
                max_subarray_sum = max_subarray_sum.max(current_max_sum);
            });
        });

        max_subarray_sum
    }
}

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
  - https://github.com/fhiyo/leetcode/pull/38/files
    同じ感じ
    あえて全探索の練習してるのさすが
  - https://github.com/erutako/leetcode/pull/3/files
    お、Step1の自分と同じ考えか？
  - https://github.com/sakupan102/arai60-practice/pull/38/files
    迷い無しという感じ

  他の想定ユースケース
  -

  改善する時にかんがえたこと
  - 余計な条件分岐をなくした
  - for_eachで解てみた
*/

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = i32::MAX;

        prices.into_iter().for_each(|price| {
            min_price = min_price.min(price);
            max_profit = max_profit.max(price - min_price);
        });
        max_profit
    }
}

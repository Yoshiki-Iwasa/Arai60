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
  - https://github.com/hayashi-ay/leetcode/pull/56/files
  - https://github.com/fhiyo/leetcode/pull/39#pullrequestreview-2165799623
  > ある日の状態はその株を持っているか持っていないかの2択。i日目に株を持っている状態の最良の収支と持っていない状態の最良の収支を考える
    この発想はなかった。利益のことを考えていて、買うときのマイナスを意識できなかった
    一日毎の状態遷移を考えるとこうなるのはそうなんだけど、思いつかなかった
    多分本番で思いつくとしたらこっちかな



  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 別の解法も書いてみる
*/

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        (1..prices.len()).for_each(|i| {
            if prices[i] > prices[i - 1] {
                max_profit += prices[i] - prices[i - 1];
            }
        });
        max_profit
    }

    // 株持ってる or 持ってないの施行を繰り返すやつ
    pub fn max_profit_2(prices: Vec<i32>) -> i32 {
        let Some(first) = prices.first() else {
            return 0; // prices is empty
        };

        let mut profit_with_stock = -*first;
        let mut profit_without_stock = 0;

        (1..prices.len()).for_each(|i| {
            profit_with_stock = std::cmp::max(profit_with_stock, profit_without_stock - prices[i]);
            profit_without_stock =
                std::cmp::max(profit_without_stock, profit_with_stock + prices[i]);
        });

        profit_without_stock
    }
}

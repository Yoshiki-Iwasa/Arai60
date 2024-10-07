// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  -

  何を考えて解いていたか
  - 何回も売買できるのか
    愚直にやるなら、すべての区間のでmax_profitを計算して、さらにすべてのmax_profitが最大になるような
    だめだこれだと、n^nになりそう

    わからず答えをみる
    なるほど、値上がりの時点で毎回売買したほうが、将来的な急騰を気にして売買しないより絶対いいのか
    図を書いて考えるべきだった

  想定ユースケース
  -

  正解してから気づいたこと
  -
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
}

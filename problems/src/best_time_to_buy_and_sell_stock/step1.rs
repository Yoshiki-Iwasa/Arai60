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
  - ある日に買った株が高く売れるかどうかは、買った日以降に買った値段より高くなっている場合のみ
    また、買った日より低い値付けの日があった時、その日までのmax_profitと低い値付けの日からみたmax_profitを比較すればいい

    max_price, min_priceを考える
    priceがmin_priceに対して、単調増加であるかぎり、max_priceを更新しつづける
    min_price以下の単調増加が壊れたら、その時点でmin_price, max_priceともに更新
    max_profit = max(max_profit, max_price - min_price)にして計算していけば答えがでる



  想定ユースケース
  -

  正解してから気づいたこと
  - よく考えれば、minは常に更新して、priceと引き算させればよかった
*/

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = i32::MAX;

        for price in prices {
            if price > min_price {
                max_profit = std::cmp::max(max_profit, price - min_price);
            } else {
                min_price = price;
            }
        }
        max_profit
    }
}

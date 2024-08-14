// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  N = amount, M = coins.length
  時間計算量: O(N * M)
  空間計算量: O(N)
*/

pub struct Solution;

impl Solution {
    const CAN_NOT_MAKE: i32 = i32::MAX;
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;

        // minimum_num_of_coins[i] is the number of coins needed to make i
        let mut minimum_num_of_coins = vec![Self::CAN_NOT_MAKE; amount + 1];
        minimum_num_of_coins[0] = 0;
        for money in 0..=amount {
            for coin in &coins {
                if money < *coin as usize {
                    continue;
                }
                let complement = money - *coin as usize;
                if minimum_num_of_coins[complement] != Self::CAN_NOT_MAKE {
                    minimum_num_of_coins[money] =
                        minimum_num_of_coins[money].min(minimum_num_of_coins[complement] + 1)
                }
            }
        }

        match minimum_num_of_coins[amount] == Self::CAN_NOT_MAKE {
            true => -1,
            false => minimum_num_of_coins[amount],
        }
    }
}

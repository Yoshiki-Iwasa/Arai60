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
  - i を作るのに必要なコイン枚数を決める方法
    0~i-1まで+1枚でちょうどiにできるようやコインがある
    この小さい方をとればいい？

    > 0~i-1まで+1枚でちょうどiにできるようやコインがある
    ここは全部見なくても引き算で良さそう




  想定ユースケース
  -

  正解してから気づいたこと
  - loopは1スタートでよかった
  - bottom upでやったけど、tow downでもよさそう
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

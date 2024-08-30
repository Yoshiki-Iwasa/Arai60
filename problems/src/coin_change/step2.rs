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
  - https://github.com/goto-untrapped/Arai60/pull/34/files
  - https://github.com/fhiyo/leetcode/pull/41/files
    最短経路問題にするのなるほど。メモリエラーになるっぽいけど発想がそもそもなかったのでなるほど




  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - top downにメモ化再起する方法も書いてみた
*/

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    const CAN_NOT_MAKE: i32 = i32::MAX;
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;

        // minimum_num_of_coins[i] is the number of coins needed to make i
        let mut minimum_num_of_coins = vec![Self::CAN_NOT_MAKE; amount + 1];
        minimum_num_of_coins[0] = 0;
        for money in 0..=amount {
            for &coin in &coins {
                if money < coin as usize {
                    continue;
                }
                let complement = money - coin as usize;
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

    pub fn coin_change_top_down(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut min_num_of_coins = vec![Self::CAN_NOT_MAKE; amount + 1];
        let mut calculated_amounts = HashSet::new();

        min_num_of_coins[0] = 0;
        calculated_amounts.insert(0);
        let num_of_coins = Self::min_coins_for_amount(
            &coins,
            amount,
            &mut min_num_of_coins,
            &mut calculated_amounts,
        );
        match num_of_coins == Self::CAN_NOT_MAKE {
            true => -1,
            false => num_of_coins,
        }
    }

    fn min_coins_for_amount(
        coins: &[i32],
        amount: usize,
        min_num_of_coins: &mut [i32],
        calculated_amounts: &mut HashSet<usize>,
    ) -> i32 {
        if amount == 0 {
            return 0;
        }
        if calculated_amounts.contains(&amount) {
            return min_num_of_coins[amount];
        }

        let mut num_of_coins = Self::CAN_NOT_MAKE;
        for coin in coins {
            if amount < *coin as usize {
                continue;
            }
            let complement = amount - *coin as usize;
            let num_coins_for_complement =
                Self::min_coins_for_amount(coins, complement, min_num_of_coins, calculated_amounts);
            if num_coins_for_complement != Self::CAN_NOT_MAKE {
                num_of_coins = num_of_coins.min(num_coins_for_complement + 1);
            }
        }
        calculated_amounts.insert(amount);
        min_num_of_coins[amount] = num_of_coins;
        num_of_coins
    }
}

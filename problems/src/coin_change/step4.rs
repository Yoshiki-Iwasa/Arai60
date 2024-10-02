// Step4
// 目的: 作れるか作れないかの配列を持ってみる

pub struct Solution;

impl Solution {
    const CAN_NOT_MAKE: i32 = -1;
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;

        // minimum_num_of_coins[i] is the number of coins needed to make i
        let mut minimum_num_of_coins = vec![i32::MAX; amount + 1];
        // can_make[i] means whether the amount can be made by coins
        let mut can_make = vec![false; amount + 1];
        can_make[0] = true;
        minimum_num_of_coins[0] = 0;
        for money in 0..=amount {
            for &coin in &coins {
                if money < coin as usize {
                    continue;
                }
                let complement = money - coin as usize;
                if can_make[complement] {
                    can_make[money] = true;
                    minimum_num_of_coins[money] =
                        minimum_num_of_coins[money].min(minimum_num_of_coins[complement] + 1);
                }
            }
        }

        match can_make[amount] {
            true => minimum_num_of_coins[amount],
            false => Self::CAN_NOT_MAKE,
        }
    }
}

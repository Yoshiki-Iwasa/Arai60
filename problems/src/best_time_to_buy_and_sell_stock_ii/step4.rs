// Step4
// 目的: 最小売買回数を求める

pub struct Solution;

impl Solution {
    // peakの数だけ売買すればいい
    pub fn num_of_transaction(prices: Vec<i32>) -> i32 {
        let mut num_of_transactions = 0;
        let mut is_up_trend = false;
        (0..prices.len() - 1).for_each(|i| {
            if prices[i] < prices[i + 1] {
                is_up_trend = true;
                return;
            }

            // peekに到達
            if is_up_trend && prices[i] > prices[i + 1] {
                is_up_trend = false;
                num_of_transactions += 1;
            }
        });
        // 最後にup trendなら売るべき
        // ここはprice.push(i32::MIN)を入れると省略できる
        if is_up_trend {
            num_of_transactions += 1
        }

        num_of_transactions
    }
}

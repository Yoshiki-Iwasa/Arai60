// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(n log (500 * 5 * 10^4))
  空間計算量: O(1)
*/

pub struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        assert!(days > 0);

        let Some(max_capacity) = weights.iter().max() else {
            // weights is empty
            return 0;
        };

        // binary search
        let mut left = *max_capacity; // minimum capacity
        let mut right = weights.iter().sum::<i32>() + 1; // max capacity + 1

        while left < right {
            let mid = left + (right - left) / 2;

            if Self::is_feasible(days, &weights, mid) {
                right = mid
            } else {
                left = mid + 1
            }
        }
        left
    }

    pub fn is_feasible(days_remaining: i32, weights: &[i32], capacity: i32) -> bool {
        let mut days_required = 1;
        let mut loading_weight = 0;

        for w in weights {
            loading_weight += w;

            if loading_weight > capacity {
                days_required += 1;
                loading_weight = *w;
            }
        }

        days_required <= days_remaining
    }
}

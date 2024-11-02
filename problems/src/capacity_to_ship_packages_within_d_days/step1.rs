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
  - 1 <= capacity <=  25 * 10^6 (500 * 5 * 10^4) なのでこの範囲で二分探索してみる
    capacityの大きさが最悪10^7オーダーで、weights.lengthが10^4オーダーだから
    最悪O(n log m) で大丈夫そう（n = weights.length, m = capacity）
    あるcapacityを設定した時days以内に出荷できるかを出力する関数を作りたい


  想定ユースケース
  -

  正解してから気づいたこと
  - leftをweights.max()にして、rightをweights.sum() + 1にするほうがnの掛け算が減ってよさそう
    こうすることで、w > capacityの心配をしなくて済む
    ただおそらく実務ではis_shippable()はライブラリとして他のところからもcallされると思う
    その場合はw > capacityを考慮したコードにするべきだと思った
*/

pub struct Solution;

impl Solution {
    fn is_shippable(days_remaining: i32, weights: &[i32], capacity: i32) -> bool {
        let mut days_required = 1;
        let mut loading_weight = 0;
        for w in weights {
            if *w > capacity {
                // capacity should be bigger than each weight
                return false;
            }

            loading_weight += w;

            if capacity < loading_weight {
                days_required += 1;
                loading_weight = *w
            }
        }
        days_required <= days_remaining
    }

    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        // binary search
        let mut left = 1; // minimum capacity
        let mut right = 500 * (5 * 1_0000) + 1; // max capacity + 1

        while left < right {
            let mid = left + (right - left) / 2;

            if Self::is_shippable(days, &weights, mid) {
                right = mid
            } else {
                left = mid + 1
            }
        }

        left
    }
}

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
  - https://github.com/goto-untrapped/Arai60/pull/41/files
  - https://github.com/rossy0213/leetcode/pull/26
  - https://github.com/fhiyo/leetcode/pull/45/files

  方針はみんな同じだが、変数名の付け方に個性が出てる





  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - daysが0の場合の考慮はStep1で抜けていたので考慮する
    探索の左端と右端を絞る

    feasible（実現可能）という言葉があるらしいのでこれを関数名につかう

    left, rightを決める時、weightsを1passで行う方法も書けるが、iterを使ったほうが読みやすいと判断


*/

pub struct Solution;

impl Solution {
    fn is_feasible(days_remaining: i32, weights: &[i32], capacity: i32) -> bool {
        let mut days_required = 1;
        let mut loading_weight = 0;
        for w in weights {
            loading_weight += w;

            if loading_weight > capacity {
                days_required += 1;
                loading_weight = *w
            }
        }
        days_required <= days_remaining
    }

    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        assert!(days > 0);

        let Some(max_weight) = weights.iter().max() else {
            // weight is empty
            return 0;
        };

        let mut left = *max_weight; // minimum capacity
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
}

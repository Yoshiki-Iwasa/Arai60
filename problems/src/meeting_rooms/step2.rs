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
  - https://github.com/Mike0121/LeetCode/pull/27/files
    入力を破壊するのは慎重に考えるか

  - https://github.com/shining-ai/leetcode/pull/55/files
    言語の標準ライブラリにwindowsのようなものがない場合、previous_endを


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 入力を破壊しないように考える
  - end_1, start_2 などは不親切なので、変数名を工夫する
  - 一応、brute_forceも書いておく。逆にbrute forceを書くのが苦手になっているフシがあるので
*/

pub struct Solution;

impl Solution {
    pub fn can_attend_meetings_1(intervals: Vec<Vec<i32>>) -> bool {
        let mut sorted_intervals = intervals.clone();
        sorted_intervals.sort_by_key(|interval| interval[0]);

        sorted_intervals.windows(2).all(|w| {
            let this_end = w[0][1];
            let next_start = w[1][0];

            this_end <= next_start
        })
    }

    // brute force
    pub fn can_attend_meetings_2(intervals: Vec<Vec<i32>>) -> bool {
        for i in 0..intervals.len() {
            for j in i + 1..intervals.len() {
                if Self::is_overlap(&intervals[i], &intervals[j]) {
                    return false;
                }
            }
        }

        true
    }

    fn is_overlap(interval1: &[i32], interval2: &[i32]) -> bool {
        (interval1[0] >= interval2[0] && interval1[0] < interval2[1])
            || (interval2[0] >= interval1[0] && interval2[0] < interval1[1])
    }
}

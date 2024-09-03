// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(N)
  空間計算量: O(1)
*/

pub struct Solution;

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        let mut sorted_intervals = intervals.clone();
        sorted_intervals.sort_by_key(|interval| interval[0]);

        sorted_intervals.windows(2).all(|w| {
            let this_end = w[0][1];
            let next_start = w[1][0];

            this_end <= next_start
        })
    }
}

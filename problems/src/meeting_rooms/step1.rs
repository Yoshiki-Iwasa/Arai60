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
  - 数直線を書いてみると,時間的に開始時間が隣り合うミーティングについて、start_1とstart_2の間にend_1がなければOK
    開始時間でintervalsをソートして、2つずつくれべれば行けそう


  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

pub struct Solution;

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_by_key(|start_end| start_end[0]); // sort by start time
        intervals.windows(2).all(|w| {
            // s is start, e is end
            let end_1 = w[0][1];
            let start_2 = w[1][0];

            end_1 <= start_2
        })
    }
}

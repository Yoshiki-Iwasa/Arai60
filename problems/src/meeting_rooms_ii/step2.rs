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
  - https://github.com/Mike0121/LeetCode/pull/28/files#diff-98fb288092e93d2ccbbd5257f188be11755b51d4a2261d80f93363938a99889fR54
    直感的な解き方でいいなあ
    ここからlistで事前にソートするなど洗練させていくのを面接でもやりたい
  - https://github.com/Mike0121/LeetCode/pull/28/files#diff-98fb288092e93d2ccbbd5257f188be11755b51d4a2261d80f93363938a99889fR122
    startとendが同じ時刻のとき、endが先に出てきてほしいのでsortの条件を明確にしたほうが良さそう


  - https://github.com/shining-ai/leetcode/pull/56/files#r1573756033
    変数名の付け方などはこれを参考にしてみよう

  - https://github.com/hayashi-ay/leetcode/pull/62/files#diff-cf7680dd4298c1c968b4f4ee68ecde49f70925564d9490c01b1c9dd7947fde87R33



  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - Eventをenumで定義してsort時のstart,endの並び順を明確にしてみる
  - min-heapにを使って終了時間を管理する方法、変数名を気をつけてみる


*/

use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    // start, endを時系列にlistに入れて処理する
    pub fn min_meeting_rooms_1(intervals: Vec<Vec<i32>>) -> i32 {
        #[derive(Clone)]
        enum Event {
            MeetingStart,
            MeetingEnd,
        }

        let mut meeting_events = intervals
            .iter()
            .map(|interval| {
                vec![
                    (interval[0], Event::MeetingStart),
                    (interval[1], Event::MeetingEnd),
                ]
            })
            .collect::<Vec<_>>()
            .concat();

        // if the time is same, MeetingEnd event comes earlier.
        meeting_events.sort_by(|(time_a, event_a), (time_b, event_b)| {
            if time_a == time_b {
                match (event_a, event_b) {
                    (Event::MeetingStart, Event::MeetingEnd) => std::cmp::Ordering::Greater,
                    (Event::MeetingEnd, Event::MeetingStart) => std::cmp::Ordering::Less,
                    _ => std::cmp::Ordering::Equal,
                }
            } else {
                time_a.cmp(time_b)
            }
        });

        let (num_required_rooms, _) = meeting_events.iter().fold(
            (0, 0), // initial value
            |(num_required_rooms, num_using_rooms), (_, event)| match event {
                Event::MeetingStart => (
                    num_required_rooms.max(num_using_rooms + 1),
                    num_using_rooms + 1,
                ),
                Event::MeetingEnd => (num_required_rooms, num_using_rooms - 1),
            },
        );
        num_required_rooms
    }

    // min-heapに入れてやってみる
    pub fn min_meeting_rooms_2(intervals: Vec<Vec<i32>>) -> i32 {
        // the earliest end comes first
        let mut end_time_p_queue = BinaryHeap::<Reverse<i32>>::with_capacity(intervals.len());

        let mut sorted_intervals = intervals;
        sorted_intervals.sort_by_key(|interval| interval[0]); // sort by start time

        sorted_intervals.iter().for_each(|interval| {
            if end_time_p_queue
                .peek()
                .is_some_and(|&Reverse(earliest_end)| earliest_end <= interval[0])
            {
                end_time_p_queue.pop();
            }
            end_time_p_queue.push(Reverse(interval[1]));
        });

        end_time_p_queue.len() as i32
    }
}

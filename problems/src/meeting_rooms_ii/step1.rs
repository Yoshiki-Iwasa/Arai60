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
  - 一番ミーティングが重なってるところがわかればいい
    Brute forceでやるなら、各intervalについて重なってる他interval数を数えてmaxをとればいい
    うまくいかず、答えをみる

  - start, endをそれぞれ独立に配列にいれて、時系列順にみていけばいいのか
    startするなら+1, endが来たら-1で単純に加減を繰り返せば良い

  - 使用可能な部屋をmin-heapで表して、start time >= heap.top なら一部屋使えるはずなのでheap.pop()
    end timeをheap.pushして最後にheapの大きさを取る



  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    // start, endを時系列に処理する
    pub fn min_meeting_rooms_1(intervals: Vec<Vec<i32>>) -> i32 {
        let (mut starts, mut ends) = intervals
            .iter()
            .map(|interval| (interval[0], interval[1]))
            .collect::<(Vec<i32>, Vec<i32>)>();

        starts.sort();
        ends.sort();

        let starts_ascending = starts;
        let ends_ascending = ends;

        let mut room_count = 0;
        let mut end_index = 0;

        (0..starts_ascending.len()).for_each(|start_index| {
            if starts_ascending[start_index] >= ends_ascending[end_index] {
                room_count -= 1;
                end_index += 1;
            }
            room_count += 1;
        });

        room_count
    }

    // min-heapで部屋を管理する解法
    pub fn min_meeting_rooms_2(intervals: Vec<Vec<i32>>) -> i32 {
        // min-heap
        let mut rooms = BinaryHeap::<Reverse<i32>>::with_capacity(intervals.len());

        let mut sorted_intervals = intervals;
        sorted_intervals.sort_by_key(|interval| interval[0]); // sort by start time

        sorted_intervals.iter().for_each(|interval| {
            if rooms
                .peek()
                .is_some_and(|&Reverse(earliest_end)| earliest_end <= interval[0])
            {
                rooms.pop();
            }

            rooms.push(Reverse(interval[1]));
        });

        rooms.len() as i32
    }
}

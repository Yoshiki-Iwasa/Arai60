// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

// より直感的な時系列にstart, endをみていく方法で解く

/*
  時間計算量: O(N logN)
  空間計算量: O(N)
*/

pub struct Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
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

        // if the time is same, MeetingEnd comes earlier
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
            (0, 0),
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
}

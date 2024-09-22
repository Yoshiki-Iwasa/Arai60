// Step4
// derive(Ord)で順序を決めてみる
// required_room_countを求める時の処理をループにしてみる

pub struct Solution;

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
enum Event {
    MeetingEnd,
    MeetingStart,
}

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
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

        meeting_events.sort();

        let mut required_room_count = 0;
        let mut using_room = 0;

        for (_, event) in meeting_events {
            match event {
                Event::MeetingEnd => using_room -= 1,
                Event::MeetingStart => {
                    using_room += 1;
                    required_room_count = required_room_count.max(using_room);
                }
            }
        }

        required_room_count
    }
}

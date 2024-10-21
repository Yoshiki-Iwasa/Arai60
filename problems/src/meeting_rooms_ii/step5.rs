use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;

#[allow(unused)]
#[derive(Clone, PartialEq, Eq)]
enum Event {
    MeetingStart,
    MeetingEnd,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Event::MeetingStart, Event::MeetingEnd) => Greater,
            (Event::MeetingEnd, Event::MeetingStart) => Less,
            _ => Equal,
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        assert!(min <= max);
        if self < Event::MeetingEnd {
            Event::MeetingEnd
        } else if self > Event::MeetingStart {
            Event::MeetingStart
        } else {
            self
        }
    }
}

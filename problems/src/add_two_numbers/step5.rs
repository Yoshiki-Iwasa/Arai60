#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

enum Carry {
    On,
    Off,
}

impl Carry {
    fn into_i32(self) -> i32 {
        match self {
            Carry::On => 1,
            Carry::Off => 0,
        }
    }
}

impl Solution {
    // Option.insert()を使ってみる

    // Option.insert(T) -> &mut T
    // なのでうまくstep3から続いていた`unwrap()`を消すことができた
    pub fn add_two_numbers_loop_sentinel_option(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode {
            val: i32::MIN,
            next: None,
        });
        let mut current = &mut sentinel;
        let mut carry = Carry::Off;
        loop {
            match (list1, list2, carry) {
                (None, None, Carry::Off) => break,
                (l1, l2, carr) => {
                    let (val1, next1) = l1.map_or((0, None), |node1| (node1.val, node1.next));
                    let (val2, next2) = l2.map_or((0, None), |node2| (node2.val, node2.next));

                    let sum: i32 = val1 + val2 + carr.into_i32();
                    carry = match sum / 10 != 0 {
                        true => Carry::On,
                        false => Carry::Off,
                    };

                    list1 = next1;
                    list2 = next2;

                    current = current.next.insert(Box::new(ListNode {
                        val: sum % 10,
                        next: None,
                    }));
                }
            }
        }

        sentinel.next
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two_numbers_loop_sentinel_option(l1, l2)
    }
}

// Step3
// 目的: レビューで示された選択肢について再考する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(n)
  空間計算量: O(n)
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

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
    // 繰り返しを使用した実装
    // sentinelをOptionにしてみる
    pub fn add_two_numbers_loop_sentinel_option(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sentinel = Some(Box::new(ListNode {
            val: i32::MIN,
            next: None,
        }));
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

                    let Some(current_node) = current.as_mut() else {
                        break;
                    };

                    current_node.next = Some(Box::new(ListNode {
                        val: sum % 10,
                        next: None,
                    }));

                    list1 = next1;
                    list2 = next2;
                    current = &mut current_node.next;
                }
            }
        }

        sentinel.unwrap().next // どちらにしてもここでunwrap()する
    }
}

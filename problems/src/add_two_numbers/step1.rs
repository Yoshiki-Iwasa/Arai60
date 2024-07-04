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
  - 直感的な解き方だと、一回整数になおして、足し合わせてまたリンクリストに戻す方法
  - 整数が100桁になる可能性があるのでこれはなし
  - 筆算をそのままやってあげればよさそう
  - 一回一回の和の操作で行いたいことは同じなので再帰的に表現できそう

  正解してから気づいたこと
  - 同じ処理を書いてるのでまとめられそう
  - carryの処理はenumを用いると安全に行えそう
  - while loopでもできそう
*/

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

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn add_number_recursive(
            list1: Option<Box<ListNode>>,
            list2: Option<Box<ListNode>>,
            node_sum: &mut Box<ListNode>,
            carry: bool, // enumで表現してもいいかも。carryは0 or 1なので
        ) {
            match (list1, list2) {
                (None, None) => {
                    if !carry {
                        return;
                    };
                    node_sum.next = Some(Box::new(ListNode { val: 1, next: None }));
                    return;
                }
                (None, Some(node2)) => {
                    let val2 = if carry { node2.val + 1 } else { node2.val };
                    let is_carry = val2 / 10 != 0;
                    node_sum.next = Some(Box::new(ListNode {
                        val: val2 % 10,
                        next: None,
                    }));
                    add_number_recursive(
                        None,
                        node2.next,
                        node_sum.next.as_mut().unwrap(),
                        is_carry,
                    );
                }
                (Some(node1), None) => {
                    let val1 = if carry { node1.val + 1 } else { node1.val };
                    let is_carry = val1 / 10 != 0;
                    node_sum.next = Some(Box::new(ListNode {
                        val: val1 % 10,
                        next: None,
                    }));
                    add_number_recursive(
                        node1.next,
                        None,
                        node_sum.next.as_mut().unwrap(),
                        is_carry,
                    );
                }
                (Some(node1), Some(node2)) => {
                    let val_sum = node1.val + node2.val;
                    let new_val = if carry { val_sum + 1 } else { val_sum };
                    let is_carry = new_val / 10 != 0;
                    node_sum.next = Some(Box::new(ListNode {
                        val: new_val % 10,
                        next: None,
                    }));

                    add_number_recursive(
                        node1.next,
                        node2.next,
                        node_sum.next.as_mut().unwrap(),
                        is_carry,
                    );
                }
            }
        }

        let mut sentinel = Box::new(ListNode {
            val: i32::MIN,
            next: None,
        });

        add_number_recursive(l1, l2, &mut sentinel, false);

        sentinel.next
    }
}

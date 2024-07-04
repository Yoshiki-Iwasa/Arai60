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
  -

  正解してから気づいたこと
  -
*/


// #[derive(PartialEq, Eq, Clone, Debug)]
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

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = ListNode { val: 0, next: head };

        let mut prev = &mut sentinel;

        while let Some(mut current_node) = prev.next.as_mut() {
            if current_node
                .next
                .as_ref()
                .is_some_and(|next_node| next_node.val == current_node.val)
            {
                while current_node
                    .next
                    .as_ref()
                    .is_some_and(|next_node| next_node.val == current_node.val)
                {
                    current_node = current_node.next.as_mut().unwrap();
                }
                prev.next = current_node.next.take();
            } else {
                prev = prev.next.as_mut().unwrap()
            }
        }

        sentinel.next
    }
}

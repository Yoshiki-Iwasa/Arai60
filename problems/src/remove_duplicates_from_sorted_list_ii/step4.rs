// Step4
// 目的: 指摘対応
// iterativeなやり方を修正してみる

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

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = ListNode { val: 0, next: head };

        let mut prev = &mut sentinel;

        while let Some(mut current_node) = prev.next.as_mut() {
            let mut is_duplicated = false;
            while current_node
                .next
                .as_ref()
                .is_some_and(|next_node| next_node.val == current_node.val)
            {
                is_duplicated = true;
                current_node = current_node.next.as_mut().unwrap();
            }

            if is_duplicated {
                prev.next = current_node.next.take();
            } else {
                prev = prev.next.as_mut().unwrap();
            }
        }

        sentinel.next
    }

    // whileが1重の場合
    pub fn delete_duplicates_single_loop(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = ListNode { val: 0, next: head };

        let mut prev = &mut sentinel;

        let mut is_duplicated = false;
        while let Some(current_node) = prev.next.as_mut() {
            if current_node
                .next
                .as_ref()
                .is_some_and(|next_node| next_node.val == current_node.val)
            {
                is_duplicated = true;
                prev.next = current_node.next.take();
                continue;
            }

            if is_duplicated {
                prev.next = current_node.next.take();
                is_duplicated = false;
            } else {
                prev = prev.next.as_mut().unwrap();
            }
        }

        sentinel.next
    }
}

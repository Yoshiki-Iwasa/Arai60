// Step4
// 目的: レビューを反映し他の選択肢も得る

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり

pub struct Solution;

// Definition for singly-linked list.

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

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;
        while let Some(current_node) = current {
            while let Some(next_node) = &mut current_node.next {
                if current_node.val != next_node.val {
                    break;
                }
                current_node.next = next_node.next.take();
            }
            current = &mut current_node.next
        }
        head
    }
}

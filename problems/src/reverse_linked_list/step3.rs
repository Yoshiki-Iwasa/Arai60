// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

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

#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse_list_recursive(
            reversed: Option<Box<ListNode>>,
            current: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            let Some(mut current_node) = current else {
                return reversed;
            };

            let next = current_node.next;
            current_node.next = reversed;
            reverse_list_recursive(Some(current_node), next)
        }
        reverse_list_recursive(None, head)
    }
}

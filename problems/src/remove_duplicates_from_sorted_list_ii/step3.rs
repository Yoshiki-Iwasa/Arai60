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
    #[allow(unused)]
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode {
            val: i32::MIN,
            next: head,
        });

        fn delete_duplicated_node(prev_node: &mut Box<ListNode>) {
            let current = &mut prev_node.next;

            if current.is_none() {
                return;
            }

            let current_node = current.as_mut().unwrap();

            let mut is_dupicated = false;

            while let Some(next_node) = &mut current_node.next {
                if current_node.val != next_node.val {
                    break;
                }
                is_dupicated = true;
                current_node.next = next_node.next.take();
            }

            if is_dupicated {
                prev_node.next = current_node.next.take();
                delete_duplicated_node(prev_node);
            } else {
                delete_duplicated_node(current_node);
            }
        }

        delete_duplicated_node(&mut sentinel);

        sentinel.next
    }
}

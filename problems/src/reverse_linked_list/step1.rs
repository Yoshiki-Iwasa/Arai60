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
  - 最も直感的な方法は値を抜き出して、逆順の別リストを作ること
  - ポインタを入れ替えて頑張る方法もあるけどそっちは可読性がかなり下がりそう
  -

  正解してから気づいたこと
  - 同じ処理の繰り返しなので再帰でできそう
  -
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

pub struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // ナイーブに解いていく
        let mut stack = vec![];
        while let Some(node) = head {
            stack.push(node.val);
            head = node.next;
        }

        let mut new_head: Option<Box<ListNode>> = None;
        let mut current = &mut new_head;
        while let Some(val) = stack.pop() {
            let Some(current_node) = current else {
                *current = Some(Box::new(ListNode { val, next: None }));
                continue;
            };

            current_node.next = Some(Box::new(ListNode { val, next: None }));
            current = &mut current_node.next;
        }

        new_head
    }
}

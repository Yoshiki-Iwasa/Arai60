// Step2
// max 30分
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時にかんがえたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  - currentとcurrent_node,nextとnext_nodeが空目しやすい

  他の人のコードを読んで考えたこと
  - 再起での解き方があるならそれもやってみるか

  改善する時にかんがえたこと
  - while let Some()でパターンマッチを単純にする
  - Option<Box<ListNode>>をなんと呼ぶべきか
  - nodeではなさそう
  - 実態としては、ポインタを格納するスタック上のメモリ or None

*/

struct Solution;

// Definition for singly-linked list.

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

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;
        while let Some(current_node) = current {
            while let Some(next_node) = &mut current_node.next {
                if next_node.val == current_node.val {
                    current_node.next = next_node.next.take();
                } else {
                    break;
                }
            }
            current = &mut current_node.next
        }
        head
    }
}

// Step2
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
  -

  他の人のコードを読んで考えたこと
  - 再帰でのアプローチも色々ある

  改善する時にかんがえたこと
  - 再帰的な実装もやってみる。
  - 処理はわりかし単純なはずなのでいけると思う
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
            if current.is_none() {
                return reversed;
            }

            let mut current_node = current.unwrap();
            let next = current_node.next;
            current_node.next = reversed;

            reverse_list_recursive(Some(current_node), next)
        }
        reverse_list_recursive(None, head)
    }
}

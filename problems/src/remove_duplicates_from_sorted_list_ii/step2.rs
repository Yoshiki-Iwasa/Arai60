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
  - 再帰的な処理の方が可読性が上がる理由はなにか？
  -

  他の人のコードを読んで考えたこと
  - 再帰的に書くアプローチを考えてみよう
  - 重複したノードを毎回のloopでやりたいことは同じなので分割統治できそう

  改善する時にかんがえたこと
  - 基底条件と部分問題はなにか
  - 入力条件から空間計算量は許容範囲か
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

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let sentinel = ListNode {
            val: i32::MIN,
            next: head,
        };

        let mut prev = Box::new(sentinel);

        fn delete_node_recursive(prev_node: &mut Box<ListNode>) {
            let current = &mut prev_node.next;
            if current.is_none() {
                return;
            }
            let mut current_node = current.as_mut().unwrap();

            let mut is_dupilicated = false;
            while let Some(next_node) = &mut current_node.next {
                if next_node.val != current_node.val {
                    break;
                }
                is_dupilicated = true;
                current_node.next = next_node.next.take()
            }

            if is_dupilicated {
                prev_node.next = current_node.next.take();
                delete_node_recursive(prev_node);
            } else {
                delete_node_recursive(&mut current_node);
            }
        }

        delete_node_recursive(&mut prev);

        prev.next
    }
}

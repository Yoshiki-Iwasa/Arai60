// Step1
// max 30分
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
  - nodeの数が高々300なので愚直にやればいい
  - -100 < node value < 100 なのでi8でOK
  - rustにはIteratorというtraitがあるのでこれを使いたいが、leetcodeでは存在しないので愚直にやる
  - borrow checkerを回避しつつListの要素を変更していくのがむずい

  正解してから気づいたこと
  - is_some()ではなくwhile let Some() で良さそう
  - expect無意味
  -
*/

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
        if let Some(current) = &mut head {
            let mut current = current;
            while current.next.is_some() {
                if current.val == current.next.as_ref().expect("next is not None").val {
                    current.next = current.next.take().unwrap().next;
                } else {
                    current = current.next.as_mut().unwrap();
                }
            }
        };
        head
    }
}

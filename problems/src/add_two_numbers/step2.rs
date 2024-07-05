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
  - 実はloopで書いた方がスッキリするか？
  - loopでの実装はmutableな変数を生みやすいのでそこを考える
    再帰とloop実装のトレードオフ
  - carryをintで表現してる人がいる
    静的型付け言語を使っているならこれを避けたい
    carryには0 or 1しか入らないのだから、これを型で表現できるならそうするべき
    carry = 0 or 1は自明だから必要ないという意見もあるかもしれない
    実際は現場の方針とトレードオフで決まると思う
    今回は表現方法の選択肢を増やす練習としてenumを使う
  - sentinelを使わない方法もあるが、余計な分岐が増えるのでsentinelでいいのではないか

  改善する時にかんがえたこと
  - 繰り返し処理の方がmutableな変数が多くなって認知コストが高い気がした
  - impl Into<i32> for Carryのみの実装は非推奨なので、Carry.into_i32()とした
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

enum Carry {
    On,
    Off,
}

impl Carry {
    fn into_i32(self) -> i32 {
        match self {
            Carry::On => 1,
            Carry::Off => 0,
        }
    }
}

impl Solution {
    // 再帰的な実装
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode {
            val: i32::MIN,
            next: None,
        });

        Self::add_two_number_recursive(l1, l2, &mut sentinel, Carry::Off);

        sentinel.next
    }

    fn add_two_number_recursive(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
        node: &mut Box<ListNode>,
        carry: Carry,
    ) {
        match (list1, list2, carry) {
            (None, None, Carry::Off) => (),
            (l1, l2, carr) => {
                let (val1, next1) = l1.map_or((0, None), |node1| (node1.val, node1.next));
                let (val2, next2) = l2.map_or((0, None), |node2| (node2.val, node2.next));

                let sum = val1 + val2 + carr.into_i32();
                let new_carry = match sum / 10 != 0 {
                    true => Carry::On,
                    false => Carry::Off,
                };

                node.next = Some(Box::new(ListNode {
                    val: sum % 10,
                    next: None,
                }));

                Self::add_two_number_recursive(
                    next1,
                    next2,
                    node.next.as_mut().unwrap(),
                    new_carry,
                );
            }
        }
    }

    // 繰り返しを使用した実装
    pub fn add_two_numbers_loop(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode {
            val: i32::MIN,
            next: None,
        });
        let mut current = &mut sentinel;
        let mut carry = Carry::Off;
        loop {
            match (list1, list2, carry) {
                (None, None, Carry::Off) => break,
                (l1, l2, carr) => {
                    let (val1, next1) = l1.map_or((0, None), |node1| (node1.val, node1.next));
                    let (val2, next2) = l2.map_or((0, None), |node2| (node2.val, node2.next));

                    let sum: i32 = val1 + val2 + carr.into_i32();
                    carry = match sum / 10 != 0 {
                        true => Carry::On,
                        false => Carry::Off,
                    };
                    current.next = Some(Box::new(ListNode {
                        val: sum % 10,
                        next: None,
                    }));

                    list1 = next1;
                    list2 = next2;
                    current = current.next.as_mut().unwrap();
                }
            }
        }

        sentinel.next
    }
}

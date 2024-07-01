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
  - sum, index1, index2 について、tupleで保存すると実態がわかりにくいらしい
    構造体を定義するとOrd と PartialOrd ２つのtraitを実装せねばならず
    コード量が増える割に恩恵はすくない

  改善する時にかんがえたこと
  - k > nums1.len() * nums2.len() だとしても動作するように保証する
  - expect() のような横着をやめる
  - 変数名をわかりやすくする
  - tupleを使いつつもtype aliasを使って意図をわかりやすくする
  - 同じような操作が多いので、関数にまとめる


  よりユーザーフレンドリーにするとしたら？
  -
  -
*/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub struct Solution;

type Sum = i32;
type IndexPiar = (usize, usize);

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        // (i32, i32, i32) = (sum, index1, index2)
        let mut heap: BinaryHeap<Reverse<(Sum, IndexPiar)>> = BinaryHeap::new();
        let mut visited: HashSet<IndexPiar> = HashSet::new();
        let mut ans = vec![];

        let (Some(n1_first), Some(n2_first)) = (nums1.first(), nums2.first()) else {
            // 空の配列が渡された場合は空配列を返す
            return vec![];
        };

        heap.push(Reverse((*n1_first + *n2_first, (0, 0))));

        visited.insert((0, 0));

        while ans.len() < k {
            let Some(Reverse((_, (index1, index2)))) = heap.pop() else {
                // k > nums1.len() * nums2.len() なら 短いansを返すことにする
                return ans;
            };

            // heapに入っていたことからindexアクセス可能なことは保証されている
            ans.push(vec![nums1[index1], nums2[index2]]);

            let pair1: IndexPiar = (index1 + 1, index2);
            Self::register_pair(&nums1, &nums2, &pair1, &mut visited, &mut heap);

            let pair2: IndexPiar = (index1, index2 + 1);
            Self::register_pair(&nums1, &nums2, &pair2, &mut visited, &mut heap);
        }
        ans
    }

    fn register_pair(
        nums1: &[i32],
        nums2: &[i32],
        pair: &IndexPiar,
        visited: &mut HashSet<IndexPiar>,
        heap: &mut BinaryHeap<Reverse<(Sum, IndexPiar)>>,
    ) {
        let (index1, index2) = pair;
        match (
            nums1.get(*index1),
            nums2.get(*index2),
            visited.contains(pair),
        ) {
            (Some(n1), Some(n2), false) => {
                let sum = n1 + n2;
                visited.insert(*pair);
                heap.push(Reverse((sum, *pair)));
            }
            _ => (),
        }
    }
}

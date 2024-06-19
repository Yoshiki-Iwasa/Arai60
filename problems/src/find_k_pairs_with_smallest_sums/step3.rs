// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  m = nums1.len()
  n = nums2.len()
  時間計算量: O(k⋅logk) or O(m⋅n⋅log(m⋅n))
  空間計算量: O(m + n)
*/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

type Sum = i32;
type IndexPair = (usize, usize);

pub struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<(Sum, IndexPair)>> = BinaryHeap::new();
        let mut visited: HashSet<IndexPair> = HashSet::new();
        let mut ans: Vec<Vec<i32>> = vec![];

        let (Some(n1), Some(n2)) = (nums1.get(0), nums2.get(0)) else {
            return vec![];
        };

        heap.push(Reverse((*n1 + *n2, (0, 0))));
        visited.insert((0, 0));

        while ans.len() < k {
            // k > nums1.len() * nums2.len() の場合短いansを返す
            let Some(Reverse((_, (index1, index2)))) = heap.pop() else {
                return ans;
            };

            // heapに入っていたことから、indexアクセスできることは保証済み
            ans.push(vec![nums1[index1], nums2[index2]]);

            let pair1 = (index1 + 1, index2);
            Self::register_pair(&nums1, &nums2, &pair1, &mut visited, &mut heap);

            let pair2 = (index1, index2 + 1);
            Self::register_pair(&nums1, &nums2, &pair2, &mut visited, &mut heap);
        }
        ans
    }

    // 共通処理はまとめておく
    fn register_pair(
        nums1: &[i32],
        nums2: &[i32],
        pair: &IndexPair,
        visited: &mut HashSet<IndexPair>,
        heap: &mut BinaryHeap<Reverse<(Sum, IndexPair)>>,
    ) {
        let (index1, index2) = pair;
        match (
            nums1.get(*index1),
            nums2.get(*index2),
            visited.contains(pair),
        ) {
            (Some(n1), Some(n2), false) => {
                let sum = *n1 + *n2;
                visited.insert(*pair);
                heap.push(Reverse((sum, *pair)));
            }
            _ => (),
        }
    }
}

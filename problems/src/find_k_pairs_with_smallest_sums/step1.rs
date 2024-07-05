// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - 最小セットを保証しながらindexをどう動かすか
  - 最小のものだけを得る方法

  何を考えて解いていたか
  - 単純な解法はすべてのペアを作りながらheapに入れること
  - でもすべてのペアを作る時点で最悪O(n^2)かかってしまうのでなし
  - ソートされてるから0, 0同士は確実に小さい
  - 一番小さいやつから一つずつindexずらしてみる


  正解してから気づいたこと
  - 変数名を長くするとよみにくいな..
  - 問題の制約条件からexpect()を使っているけど、これは良くない
*/

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((nums1[0] + nums2[0], 0, 0)));

        let mut visited = HashSet::new();

        visited.insert((0, 0));

        let mut ans = vec![];

        while ans.len() < k && !heap.is_empty() {
            // 最小のsumのときの情報をpopする
            let Reverse((_, nums1_index, nums2_index)) = heap.pop().expect("ensured by heap.len()");

            ans.push(vec![nums1[nums1_index], nums2[nums2_index]]);

            let num1_next_index = nums1_index + 1;
            if num1_next_index < nums1.len() && !visited.contains(&(num1_next_index, nums2_index)) {
                visited.insert((num1_next_index, nums2_index));
                let sum = nums1[num1_next_index] + nums2[nums2_index];
                heap.push(Reverse((sum, num1_next_index, nums2_index)))
            }

            let num2_next_index = nums2_index + 1;
            if num2_next_index < nums2.len() && !visited.contains(&(nums1_index, num2_next_index)) {
                visited.insert((nums1_index, num2_next_index));
                let sum = nums1[nums1_index] + nums2[num2_next_index];
                heap.push(Reverse((sum, nums1_index, num2_next_index)))
            }
        }
        ans
    }
}

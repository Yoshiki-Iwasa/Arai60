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
  - frequency or count
    frequencyにしたほうが興味の対象がわかりやすいからいいかも

  - quick select 知らなかった
    quick selectで解くなら、未ソートの配列にどんどん要素がたされていく中でkthを探すときかな

  - heapで解くと、時間計算量: O(Nlogk)
    mapでカウント -> max-heapに詰め替える -> popしていく
    map -> vec -> sort() -> 先頭k個 取り出しが自然な気がするけど、kの設定次第か

  改善する時にかんがえたこと
  - 変数名にもう少し気をつかってみるか
*/

pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    // O(nlogn)
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut frequency_map = HashMap::<i32, usize>::new();

        // O(n)
        for n in nums {
            if let Some(cnt) = frequency_map.get_mut(&n) {
                *cnt += 1;
            } else {
                frequency_map.insert(n, 1);
            }
        }

        let mut frequency_vec = frequency_map.iter().collect::<Vec<(_, _)>>();

        // 内部的には与えた関数に従ってマージソートが行われる
        frequency_vec.sort_by(|(_, cnt_a), (_, cnt_b)| cnt_b.cmp(cnt_a));

        let (top_k, _) = frequency_vec.split_at(k as usize);

        top_k.iter().map(|(val, _)| **val).collect::<Vec<_>>()
    }

    // heapを使った実装
    // O(nlogk)
    pub fn top_k_frequent_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut frequency_map = HashMap::<i32, usize>::new();

        for n in nums {
            if let Some(cnt) = frequency_map.get_mut(&n) {
                *cnt += 1;
            } else {
                frequency_map.insert(n, 1);
            }
        }

        // RustではTupleは先頭要素から比較されていくため、ヒープ上ではnumとfreqを入れ替えて管理する
        let mut frequency_heap = BinaryHeap::<Reverse<(usize, i32)>>::new();

        frequency_map.into_iter().for_each(|(num, freq)| {
            frequency_heap.push(Reverse((freq, num)));
            if frequency_heap.len() > k {
                frequency_heap.pop();
            }
        });

        frequency_heap
            .into_vec()
            .iter()
            .map(|Reverse((_, num))| *num)
            .collect::<Vec<_>>()
    }
}

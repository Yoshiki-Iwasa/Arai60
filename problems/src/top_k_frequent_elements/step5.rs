// Step5

pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut frequency_map: HashMap<i32, usize> = HashMap::new();

        for n in nums {
            // std::collections::hash_map::Entry の or_insert() を使用
            *frequency_map.entry(n).or_insert(0) += 1;
        }

        let mut frequency_vec = Vec::<(i32, usize)>::from_iter(frequency_map);

        frequency_vec.sort_by(|(_, a_freq), (_, b_freq)| b_freq.cmp(a_freq));

        let (top_k, _) = frequency_vec.split_at(k);

        top_k.iter().map(|(num, _)| *num).collect::<Vec<_>>()
    }

    pub fn top_k_frequent_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let mut frequency_map: HashMap<i32, usize> = HashMap::new();

        for n in nums {
            // std::collections::hash_map::Entry の or_insert() を使用
            *frequency_map.entry(n).or_insert(0) += 1;
        }

        // Rustではtupleは先頭から評価されるので(freq, num)にしてヒープを構成する
        let mut frequency_heap = BinaryHeap::<Reverse<(usize, i32)>>::new();

        frequency_map.into_iter().for_each(|(num, freq)| {
            frequency_heap.push(Reverse((freq, num)));
            if frequency_heap.len() > k {
                frequency_heap.pop();
            }
        });

        frequency_heap
            .iter()
            .map(|Reverse((_, num))| *num)
            .collect::<Vec<_>>()
    }
}

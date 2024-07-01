// Step4
// 目的: 指摘対応
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.iter().collect::<HashSet<_>>();
        let nums2_set = nums2.iter().collect::<HashSet<_>>();

        // 標準ライブラリの関数で解いてみる
        nums1_set
            .intersection(&nums2_set)
            .map(|n| **n)
            .collect::<Vec<_>>()
    }
}

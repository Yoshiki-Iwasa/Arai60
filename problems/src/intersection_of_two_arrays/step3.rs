// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  n = nums1.len()
  m = nums2.len()
  時間計算量: O(n + m)
  空間計算量: O(n + m)
*/

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.iter().collect::<HashSet<_>>();
        let mut intersection_set = HashSet::new();

        for n2 in nums2 {
            if nums1_set.contains(&n2) {
                intersection_set.insert(n2);
            }
        }
        intersection_set.into_iter().collect::<Vec<_>>()
    }

    pub fn intersection_two_sets(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.iter().collect::<HashSet<_>>();
        let nums2_set = nums2.iter().collect::<HashSet<_>>();

        let mut intersection = vec![];
        for n1 in nums1_set {
            if nums2_set.contains(n1) {
                intersection.push(*n1)
            }
        }
        intersection
    }
}

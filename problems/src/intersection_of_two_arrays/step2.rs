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

  他の人のコードを読んで考えたこと
  - sortして両方のindexを進めていく方法もある
    あまりメリットがわからない
- 変数名はintersection_setの方が良い家




  改善する時にかんがえたこと
  - setを２つ用意する必要はない。（してもいい。時間計算量も空間計算量も変わらない）
  - 個人的には、両方setを用意したほうが好き
  - メソッドチェーンでも解けそうなので、それも書いてみる
*/

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.iter().collect::<HashSet<_>>();
        let mut intersection_set = HashSet::<i32>::new();

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

    pub fn intersection_functional(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.iter().collect::<HashSet<_>>();

        nums2
            .into_iter()
            .fold(HashSet::<i32>::new(), |mut intersection_set, n2| {
                if nums1_set.contains(&n2) {
                    intersection_set.insert(n2);
                }
                intersection_set
            })
            .into_iter()
            .collect::<Vec<_>>()
    }
}

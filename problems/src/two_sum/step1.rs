// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - 補数という概念がでてこなかった
  - 手続き的に引き算すればいいのかな？となった

  何を考えて解いていたか
  - ん？これはナイーブに見ていくしかないのか？
  - 二重ループにはしたくない
  - 引き算していけばいい？

  正解してから気づいたこと
  - 一応O(n^2)でも10^8だから間に合わないことはない
  - これ補数だった
  - 2の補数はbit演算で使うけどそれ以外で意識したことなかった
*/

pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();
        for (index, n) in nums.iter().enumerate() {
            match map.get(n) {
                Some(other_index) => return vec![index as i32, *other_index as i32],
                None => map.insert(target - n, index),
            };
        }
        // 問題設定からここには到達しない
        vec![]
    }
}

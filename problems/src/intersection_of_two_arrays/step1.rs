// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  -

  何を考えて解いていたか
  - 自分がこれを手作業でやれと言われたときにどうするか
    まずnum1,num2から重複を削除する
    その後で、num1, num2合わせて重複してるやつだけを取り出す


  正解してから気づいたこと
  - どうせ答えはユニークにするんだから、nums1_set と answer_setだけで十分かも
*/

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.iter().collect::<HashSet<_>>();
        let nums2_set = nums2.iter().collect::<HashSet<_>>();

        let mut ans = vec![];
        for n1 in nums1_set {
            if nums2_set.contains(n1) {
                ans.push(*n1)
            }
        }
        ans
    }
}

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
  - HashMapで出現回数を記録する O(logn)
  - あとはsortしてk個取り出せばOK
  - 全体計算量はO(n)で済むかな
  - heap と priority queueがテーマだったきがするのでそれ使うともっとうまくできる？

  正解してから気づいたこと
  - 一気にiteratorをつなげてきれいにもできそう
  -
*/

// 1 <= nums.length <= 10^5
// -10^4 <= nums[i] <= 10^4
// k is in the range [1, the number of unique elements in the array].
// It is guaranteed that the answer is unique.

pub struct Solution;

use std::collections::{HashMap};
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();

        for n in nums {
            if let Some(cnt) = map.get_mut(&n) {
                *cnt += 1;
            } else {
                map.insert(n, 1);
            }
        }
        let mut vec = map.iter().collect::<Vec<(_, _)>>();
        vec.sort_by(|(_, cnt_a), (_, cnt_b)| cnt_b.cmp(cnt_a));
        let (top_k, _) = vec.split_at(k as usize);
        top_k.iter().map(|(val, _)| **val).collect::<Vec<_>>()
    }
}

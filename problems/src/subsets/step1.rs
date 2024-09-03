// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - Rustのbit演算になれてなくて、書き方を少し調べた

  何を考えて解いていたか
  - N個の中からいくつか選ぶならbit全探索でいけそう

  想定ユースケース
  -

  正解してから気づいたこと
  - バックトラッキングなどの方法もある
*/

pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        (0_u32..(1 << nums.len()))
            // numsのcombinationをbitのON/OFFで表現する
            .map(|bit_pattern| {
                (0..nums.len())
                    // i番目のbitが立っている場合はそれを採用する
                    .flat_map(|i| match bit_pattern & (1 << i) != 0 {
                        true => Some(nums[i]),
                        false => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

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
  - max_amounts[i]をiの家を含めて最大でいくら取れるかとする
    max_amounts[i] = max(max_amounts[0~i-2]) + nums[i]
    これだとO(N^2)かかる
    N <= 100だからこれでもOKか
    でも全て比較する必要はなくて、i-2, i-3どっちを採用するかを考えれば十分なので
    O(N)でいけそう

  想定ユースケース
  -

  正解してから気づいたこと
  - 最大限お金を盗むならいくらなになるかという試行をお尻からやるという発想がなかった
    メモ化再帰でも良さそう
*/

pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut max_amounts = vec![0; nums.len()];

        (0..nums.len()).for_each(|i| match i {
            0 => max_amounts[0] = nums[0],
            1 => max_amounts[1] = nums[1],
            2 => max_amounts[2] = max_amounts[0] + nums[2],
            _ => {
                // except adjacent house
                max_amounts[i] =
                    std::cmp::max(max_amounts[i - 2] + nums[i], max_amounts[i - 3] + nums[i]);
            }
        });
        // return 0 if nums is empty
        max_amounts.into_iter().max().unwrap_or_default()
    }
}

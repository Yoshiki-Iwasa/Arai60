// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - 配列の要素をうまく入れ替えてremoveなどなしでやる実装

  何を考えて解いていたか
  - 愚直にやるなら、nums[i]が0であるかぎり消していく
    Optionの機能を使えば安全に消していけるはず
    その後、消した数だけ0を足してやればいい


  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_cnt = 0;
        (0..nums.len()).for_each(|i| {
            while nums.get(i).is_some_and(|&n| n == 0) {
                nums.remove(i);
                zero_cnt += 1;
            }
        });
        nums.extend(vec![0; zero_cnt]);
    }
}

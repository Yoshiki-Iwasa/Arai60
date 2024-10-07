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
  - 今度は家が連結している
    とりあえず、つながってない想定で解いてみて、そのあとこの条件について調整を行う
    0, n - 1がないバージョンでそれぞれ同じことやって、大きい方を選ぶ？

  想定ユースケース
  -

  正解してから気づいたこと
  - これもHouse Robberと同様に配列の末尾だけ使うやつをやればいい
*/
pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        std::cmp::max(
            Self::rob_helper(&nums[1..]),
            Self::rob_helper(&nums[0..nums.len() - 1]),
        )
    }

    fn rob_helper(nums: &[i32]) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut max_amounts = vec![0; nums.len() + 1];

        max_amounts[nums.len()] = 0;
        max_amounts[nums.len() - 1] = nums[nums.len() - 1];

        (0..nums.len() - 1).rev().for_each(|i| {
            max_amounts[i] = std::cmp::max(max_amounts[i + 1], max_amounts[i + 2] + nums[i]);
        });
        max_amounts[0]
    }
}

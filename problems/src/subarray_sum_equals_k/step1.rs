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
  - 累積和で解けそう
  - 累積和の組み合わせを回すので、O(n^2)でだめだけど、10^8でギリ許容か
  -

  想定ユースケース
  - 住んでいる人口がkになるような駅の区間がいくつあるか。どこにあるか。
    それをマーケチームから全国の路線図を渡されて解析するときにスクリプトの一部として存在しそう
    それかバッチとして定期実行されるか
    もし厳密なエラー処理をするなら、log fatalさせてエンジニアに早く伝わるようにしたほうがいいかも

  正解してから気づいたこと
  - 和を考えるときは補数を考えてみる発想が必要だった
*/
pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cumulative_sum = Vec::<i32>::with_capacity(nums.len());
        cumulative_sum.push(0);
        for (index, n) in nums.iter().enumerate() {
            cumulative_sum.push(cumulative_sum[index] + n)
        }

        let mut cnt = 0;

        for start in 0..nums.len() {
            for end in start + 1..=nums.len() {
                if cumulative_sum[start] - cumulative_sum[end] == k {
                    cnt += 1;
                }
            }
        }

        cnt
    }
}

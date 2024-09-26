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
  - まず、permutationの生成方法については知らない
    numsの中の数字はuniqueとな
    nums.len <= 6なので雑な全探索をやってそこから修正してもいいはず
    素直に順列を作るなら樹形図を書くので、これをプログラム上で書くイメージでやってみる
    stackに[[nums[0]]~nums[[last]]]を用意して、樹形図を深さ優先探索する感じか？

    枝の分岐をするときにnumsを毎回走査しないといけない
    n*n!の時間計算量
    numsに重複がある場合はどうするか
    数字とindexを同時に管理して、permutationをつくりつつ最後にsetに落とし込んでやるか



  想定ユースケース
  -

  正解してから気づいたこと
  - Backtrackingでやるとかっこよくできるのか
  - RustのitertoolのPermutation実装
    https://docs.rs/itertools/latest/src/itertools/permutations.rs.html
*/

pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut stack = vec![];
        nums.iter().for_each(|n| stack.push(vec![*n]));

        let mut permutations = vec![];

        while let Some(instance) = stack.pop() {
            if instance.len() == nums.len() {
                permutations.push(instance);
                continue;
            }

            nums.iter().for_each(|n| {
                if !instance.contains(n) {
                    let mut cloned = instance.clone();
                    cloned.push(*n);
                    stack.push(cloned)
                }
            })
        }

        permutations
    }
}

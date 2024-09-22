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
  - どっかでpermutationを解いたとき、next_permutationをつかった解法をやったのでそれを思い出しながら
    最後に昇順になってるindexを取得する。
    そのindexより最右でおおきいやつを取得
    swap
    reverse

  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut [i32]) {
        // 昇順になる最右のindexを求める
        let Some(pivot_index) = nums.windows(2).rposition(|w| w[0] < w[1]) else {
            // 存在しない場合、降順なので反転させて返す
            nums.reverse();
            return;
        };

        let Some(rightmost_successor_index) = nums.iter().rposition(|&n| n > nums[pivot_index])
        else {
            // pivot_indexがある場合、かならずそれより大きいindexは存在する
            unreachable!()
        };

        // numsを少しだけ大きくする
        nums.swap(pivot_index, rightmost_successor_index);
        // 降順部分を昇順にする
        nums[pivot_index + 1..].reverse();
    }
}

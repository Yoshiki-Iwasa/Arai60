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
  - 明らかに前の状況がきいてきそう
    post[i]を塗るとき、post[i - 1]と違う色なら思考停止で塗ってOK
    同じだと、post[i - 2]の状況が必要か

    post[i]を知るためには、post[i-1], post[i-2]の情報が必要
    post[i]を塗る方法をways[i]とすると、
    post[i]をpost[i-1]と違う色で塗る方法は: ways[i - 1] * k - 1
    post[i]をpost[i-1]と同じ色でぬる方法は: ways[i - 1] * 1通り
      でも、これはpost[i-2]がpost[i-1]と違う色の場合なので、ways[i - 2] * (k - 1)


    ways[i] = ways[i - 1] * k - 1 + ways[i - 2] * (k - 1) = (ways[i-1] + ways[i - 2])*(k - 1)
    i == 0 k通り
    i == 0 なら k * k 通り











  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

pub struct Solution;

impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        // ways_to_paint[i] means the number of ways to paint i-th post
        let mut ways_to_paint = Vec::<i32>::with_capacity(n as usize);

        let n = n as usize;

        (0..n).for_each(|i| {
            if i == 0 {
                ways_to_paint.push(k);
                return;
            }
            if i == 1 {
                ways_to_paint.push(k * k);
                return;
            }
            ways_to_paint.push((ways_to_paint[i - 2] + ways_to_paint[i - 1]) * (k - 1));
        });

        ways_to_paint[n - 1]
    }
}

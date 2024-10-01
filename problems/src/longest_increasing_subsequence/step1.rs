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
  - 前の状況を使って、うまいこと進めないといけない
    愚直にやるなら、先頭から一個ずつ見ていって。これを先頭をずらしながらやる
    dpテーブル使って、maxとってやればいい
    でもこれだとO(n^2)になるので無し
    [ 0,1,2,3,4,5,  6, 7]
    [10,9,2,5,3,7,101,18]
    ↓
    [2,4,3,5,1, 0, 7,  6]
    [2,3,5,7,9,10,18,101]

    ソートして、indexが単調増加してる区間を探すのはどうだろう
    O(nlogn)で終わりそう
    重複あった途端に破綻だ

    答えを見る
    N^2 <= 10^8だから、最悪愚直なDPをやるのもありだったのか

    O(n logn)にするには、numsに出てくる要素nをsubに保存していく
    n > sub.max()ならpush, そうでなければ、subの要素sに対し、n >= s になる要素と入れ替える



  想定ユースケース
  -

  正解してから気づいたこと
  - 最後の`dp.iter().max()`はなくてもdpの更新中にできそう
    O(n^2)アルゴリズムの中でそれがどのくらい嬉しいかはさておき

    1がマジックナンバーになってるかも。
*/

pub struct Solution;

impl Solution {
    // O(n^2)のDPで書いてみる
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // dp[i] is the length of the longest increasing subsequence that ends with the i-th element
        let mut dp = vec![1; nums.len()];

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1)
                }
            }
        }

        let Some(max) = dp.iter().max() else {
            panic!("nums is empty")
        };
        *max
    }
}

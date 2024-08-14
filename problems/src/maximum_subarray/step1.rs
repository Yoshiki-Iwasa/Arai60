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
  - 累積和かな
    prefix_sum[r] - prefix_sum[l]を全区間で見ていく
    O(N^2)かかるけど、とりあえず解けはしそう
    と思ったら、N^2 <= 10^10でだめそう

    答えを見る
    Kadane's algorithmなるものが説明されている
    知らなかった
    今見ている部分列を更新するか新しい部分列を開始するかは、今見ている部分列の和と次の要素との関係できめればいい
    現在の部分列+次の要素より次の要素単体が大きいのなら部分列をそこで捨てれば良い
    そうでなければ拡張すればいい


  想定ユースケース
  -

  正解してから気づいたこと
  - 累積和で空間を閉めなくても、loopのたびにcurrent_sumをつくればそれでよさそう
  - numsが空のときにpanicさせるのはあまりにも雑だと思った
    max_sub_arrayがほしいなら何も選ばないという意味で0だろうか
*/

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let Some(first) = nums.first() else {
            panic!("nums is empty")
        };
        let mut max_current_sum = *first;
        let mut max_subarray_sum = *first;

        (1..nums.len()).for_each(|i| {
            // 現在の部分列を継続するか、新しくするかを決定
            max_current_sum = std::cmp::max(nums[i], max_current_sum + nums[i]);
            max_subarray_sum = std::cmp::max(max_subarray_sum, max_current_sum);
        });

        max_subarray_sum
    }

    // brute fource (TLE)
    pub fn max_sub_array_tle(nums: Vec<i32>) -> i32 {
        let prefix_sum = std::iter::once(&0)
            .chain(nums.iter())
            .scan(0, |state, n| {
                *state += n;
                Some(*state)
            })
            .collect::<Vec<_>>();

        let mut max = i32::MIN;
        for left in 0..=nums.len() {
            for right in (left + 1)..=nums.len() {
                max = max.max(prefix_sum[right] - prefix_sum[left])
            }
        }

        max
    }
}

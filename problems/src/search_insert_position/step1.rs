// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - 二分探索、毎回ちょっと+1するべきかとか考えてしまう


  何を考えて解いていたか
  - 二分探索
    今回欲しいのはupper bound
    なのでleftを返す想定でいくことにする
    最終的にleftを返すので、rightを開区間にして実装スタート

    while条件は、left < right (leftとrightが追いついたとこで終わり)
    最終的にleftを返さないといけないのだから、leftは mid + 1 にしないといけない(target未満の値に+1しないと答えにたどり着けない)


    標準ライブラリにすでに二分探索があった
    せっかくなのでそれをつかってstep2で解く



  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid,
                std::cmp::Ordering::Equal => {
                    return mid as i32;
                }
            }
        }
        left as i32
    }
}

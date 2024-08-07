// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - sortedもrotatedも同時に扱おうとして沼にハマる
    答えの冒頭をみて、sortedとrotatedを分けているのを見てやりなおし


  何を考えて解いていたか
  - 昇順が壊れるポイントを探したい
    ソート済みだったらnums[0]
    rotatedだったらleft, rightともに閉区間にして、left < mid ならleft = mid, right > mid なら rightを移動
    最終的にrightを返せばOK




  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

pub struct Solution;

impl Solution {
    fn is_rotated(nums: &[i32]) -> bool {
        let first = nums[0];
        let last = nums[nums.len() - 1];
        first > last
    }

    pub fn find_min(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());

        if !Self::is_rotated(&nums) {
            return nums[0];
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        while right - left > 1 {
            let mid = left + (right - left) / 2;

            if nums[mid] > nums[left] {
                left = mid
            }

            if nums[mid] < nums[right] {
                right = mid
            }
        }
        nums[right]
    }
}

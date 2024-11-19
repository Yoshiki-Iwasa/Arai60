// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - pivotで分割せずに一発でうまくやるクエリがおもいついかなかった


  何を考えて解いていたか
  - まず与えられた配列とtargetで、どのようにtrue,false配列を作るかを考える
    pivotで配列を２つにわけて、それぞれに対して二分探索をすればいいのでは？


  想定ユースケース
  -

  正解してから気づいたこと
  - pivotが決まれば、nums[pivot]と２つの配列の比較でどちらに入っているかわかるのでは？
    あとはfind targetして、offsetを加えれば終わりでは？


*/

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let pivot = Self::find_pivot(&nums);

        let (nums_head, nums_tail) = (&nums[0..pivot], &nums[pivot..]);

        // ここは標準ライブラリのvec.binary_searchでも同様に書けるが、練習のため自分で書く
        match (
            Self::find_target_index(nums_head, target),
            Self::find_target_index(nums_tail, target),
        ) {
            (Ok(_), Ok(_)) => unreachable!(),
            (Ok(index), Err(_)) => index as i32,
            (Err(_), Ok(index)) => (index + nums_head.len()) as i32, // index + offset
            (Err(_), Err(_)) => -1,
        }
    }

    fn find_target_index(nums: &[i32], target: i32) -> Result<usize, ()> {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return Ok(mid);
            }

            if nums[mid] > target {
                right = mid
            } else {
                left = mid + 1
            }
        }

        Err(())
    }

    fn find_pivot(nums: &[i32]) -> usize {
        let mut left = 0;
        let mut right = nums.len();

        let last = nums[nums.len() - 1];

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] <= last {
                right = mid
            } else {
                left = mid + 1
            }
        }
        left
    }
}

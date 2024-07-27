// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(log n)
  空間計算量: O(1)
*/

// 多分自分が面接で書くとしたらis_rotatedを確認すると思う
// 直感的にもrotatedとそうでない場合は分けたくなると思うので分ける実装も残しておく

pub struct Solution;

impl Solution {
    fn is_rotated(nums: &[i32]) -> bool {
        let first = nums[0];
        let last = nums[nums.len() - 1];
        first > last
    }

    // rotatedを判定するやつ
    pub fn find_min_1(nums: Vec<i32>) -> i32 {
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

    // rotatedを確認しないやつ
    pub fn find_min_2(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] > nums[nums.len() - 1] {
                left = mid + 1
            } else {
                right = mid
            }
        }

        nums[left]
    }
}

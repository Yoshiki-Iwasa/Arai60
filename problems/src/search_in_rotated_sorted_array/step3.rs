// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(1)
  空間計算量: O(log n)
*/

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] <= nums[mid] {
                // left is sorted

                if nums[left] <= target && target < nums[mid] {
                    // target exists in left~mid
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // right is sorted

                if nums[mid] < target && target <= nums[right] {
                    // target exist in mid~right
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
}

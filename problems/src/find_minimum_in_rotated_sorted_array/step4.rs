// Step4
// 目的:https://github.com/Yoshiki-Iwasa/Arai60/pull/35#discussion_r1693974411

// 上記コメントを受けて、入力に対して、true, falseを吐くクエリを用意して、true, falseの境界を探すということでやってみる

use core::num;

pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        let mut left = 0;
        let mut right = nums.len();

        let last = nums[nums.len() - 1];

        while left < right {
            let mid = left + (right - left) / 2;

            // nums[mid] > lastをqueryとすると
            // midがFalseの場合、leftがmid+1になってTrueになる可能性に向かう
            // midがTrueの場合、その場所は探索に含む必要がないのでright = midとする
            // このまま進むと、leftが境界のTrue側にたどり着いたとき、あとはrightがleftに追いつくまで探索がつづいて終わり
            if nums[mid] > last {
                left = mid + 1
            } else {
                right = mid
            }
        }
        nums[left]
    }
}

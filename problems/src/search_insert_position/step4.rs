// Step4
// https://github.com/Yoshiki-Iwasa/Arai60/pull/34#discussion_r1689479461
// Python の bisect_{left,right} を参考に、early returnせずにやってみる

pub struct Solution;

impl Solution {
    pub fn search_insert_bitsect_left(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] < target {
                left = mid + 1
            } else {
                right = mid
            }
        }
        left as i32
    }
}

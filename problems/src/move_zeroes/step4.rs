// Step4
// https://github.com/Yoshiki-Iwasa/Arai60/pull/59/files/95ddc14a1984729c672cb2602d74438b0250f6f6#r1747414033
// これを受けて、erase idiom っぽいことをやってみる
// Vec::retainは一度だけ要素を訪れて、条件に会うものだけを残す

pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let original_len = nums.len();
        nums.retain(|&n| n != 0);
        nums.resize(original_len, 0);
    }
}

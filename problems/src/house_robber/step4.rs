pub struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let Some(first) = nums.first() else {
            return 0;
        };

        let mut max_before = 0;
        let mut max_so_far = *first;

        (1..nums.len()).for_each(|i| {
            // この家から盗むパターンと盗まないパターン
            let current_max = std::cmp::max(max_so_far, max_before + nums[i]);
            max_before = max_so_far;
            max_so_far = current_max;
        });
        max_so_far
    }
}

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        std::cmp::max(
            Self::rob_helper(&nums[1..]),
            Self::rob_helper(&nums[0..nums.len() - 1]),
        )
    }

    fn rob_helper(nums: &[i32]) -> i32 {
        let Some(first) = nums.first() else {
            return 0; // nums is empty
        };

        let mut max_so_far = *first;
        let mut max_before = 0;

        (1..nums.len()).for_each(|i| {
            let max_sum = std::cmp::max(max_so_far, max_before + nums[i]);
            max_before = max_so_far;
            max_so_far = max_sum
        });
        max_so_far
    }
}

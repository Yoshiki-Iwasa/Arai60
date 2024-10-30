// Step4

pub struct Solution;

impl Solution {
    // backtracking
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::subsets_helper(&nums, vec![], &mut result);
        result
    }

    fn subsets_helper(nums: &[i32], mut subset: Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(subset.clone());

        nums.iter().enumerate().for_each(|(index, &n)| {
            subset.push(n);
            Self::subsets_helper(&nums[index..], subset.clone(), result);
            subset.pop();
        })
    }
}

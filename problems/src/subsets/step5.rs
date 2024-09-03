// Step5
// 余計なcloneを消す

pub struct Solution;

impl Solution {
    // backtracking
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::generate_subsets(&nums, &mut vec![])
    }

    fn generate_subsets(nums: &[i32], subset: &mut Vec<i32>) -> Vec<Vec<i32>> {
        let mut subset_of_nums = vec![];
        subset_of_nums.push(subset.to_vec());

        nums.iter().enumerate().for_each(|(index, &n)| {
            subset.push(n);
            subset_of_nums.extend(Self::generate_subsets(&nums[index..], subset));
            subset.pop();
        });
        subset_of_nums
    }
}

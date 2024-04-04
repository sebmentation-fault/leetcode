pub struct Solution;

impl Solution {
    /// Given an array of integers `nums` and an integer `target`,
    /// return indices of the two numbers such that they add up to `
    /// target`.
    ///
    /// This solution is O(n^2). Better solutions make use of a HashMap,
    /// to instantly find the value (if any) of the required value.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();

        for i in 0..n-1 {
            let r: i32 = target - nums[i];

            for j in i+1..n {
                if r == nums[j] {
                    return vec![i as i32, j as i32];
                }
            }
        }

        return vec![];
    }
}

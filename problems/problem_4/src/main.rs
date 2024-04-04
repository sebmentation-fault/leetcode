mod solution;
use solution::Solution;

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("{:?}", result); // Expected output: 2.00000
}

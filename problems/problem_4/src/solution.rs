pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total: usize = nums1.len() + nums2.len();
        let is_even: bool = total % 2 == 0;
        let mut last: i32 = 0;
        let mut snd_last: i32 = 0;
        let mut i: usize = 0;
        let mut j: usize = 0;

        for _k in 0..=(total/2) {
            snd_last = last;

            if j >= nums2.len() || (i < nums1.len() && nums1[i] < nums2[j]) {
                last = nums1[i];
                i += 1;
            } else {
                last = nums2[j];
                j += 1;
            }
        }

        if is_even { (last + snd_last) as f64 / 2.0 } else { last as f64 }
    }
}

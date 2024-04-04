mod solution;
use solution::Solution;

fn main() {
    let s: String = "abcabcbb".to_string();
    let result = Solution::length_of_longest_substring(s);
    println!("{:?}", result); // Expected output: 3

}

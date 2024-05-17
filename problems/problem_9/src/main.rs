mod solution;
use solution::Solution;

fn main() {
    let x = 100000001;
    let result = Solution::is_palindrome(x);
    println!("{:?}", result); // Expected output: true
}

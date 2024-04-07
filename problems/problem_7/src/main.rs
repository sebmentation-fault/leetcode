mod solution;
use solution::Solution;

fn main() {
    let x = 123;
    let result = Solution::reverse(x);
    println!("{:?}", result); // Expected output: 321
}

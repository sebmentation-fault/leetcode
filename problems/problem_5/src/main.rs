mod solution;
use solution::Solution;

fn main() {
    let s = "babad".to_string();
    let result = Solution::longest_palindrome(s);
    println!("{:?}", result);
}

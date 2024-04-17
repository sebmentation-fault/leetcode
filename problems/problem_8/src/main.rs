mod solution;
use solution::Solution;

fn main() {
    let x = String::from("123");
    let result = Solution::my_atoi(x);
    println!("{:?}", result); // Expected output: 123
}

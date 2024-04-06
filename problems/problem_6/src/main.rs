mod solution;
use solution::Solution;

fn main() {
    let s = String::from("PAYPALISHIRING");
    let n = 3;
    let result = Solution::convert(s, n);
    println!("{:?}", result);
}

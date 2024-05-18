mod solution;
use solution::Solution;

fn main() {
    let s = "aa";
    let p = "*";
    let result = Solution::is_match(s.into(), p.into());
    println!("{:?}", result);
}

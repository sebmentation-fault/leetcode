pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest: usize = 0;
        let mut substr: String = String::new();

        for c in s.chars() {
            while substr.contains(c) {
                substr.remove(0);
            }

            substr.push(c);

            longest = longest.max(substr.len());
        }

        longest as i32
    }
}

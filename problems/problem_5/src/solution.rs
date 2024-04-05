pub struct Solution;

impl Solution {

    /// Not my solution, but originally was written in C,
    /// so I've attempted to reproduce the algorithm in
    /// Rust.
    pub fn longest_palindrome(s: String) -> String {
        let mut max_len = 0;
        let mut start = 0;
        // let s_bytes = s.as_bytes(); // Work directly with bytes for ASCII compatibility

        for i in 0..s.len() {
            let len1 = Self::extend(&s, i as isize, i as isize);
            let len2 = Self::extend(&s, i as isize, i as isize + 1);
            let len = len1.max(len2);

            if len > max_len {
                max_len = len;
                start = i - (len - 1) / 2;
            }
        }

        s[start as usize..start as usize + max_len].to_string()
    }

    fn extend(s: &str, mut start: isize, mut end: isize) -> usize {
        while start >= 0 && end < s.len() as isize && s.as_bytes()[start as usize] == s.as_bytes()[end as usize] {
            start -= 1;
            end += 1;
        }

        (end - start - 1) as usize
    }
}

struct _Solution;

impl _Solution {
    pub fn _longest_palindrome(s: String) -> String {
        let mut longest = String::new();

        for i in 0..s.len() {
            for j in i..s.len() {
                let substr = &s[i..=j];
                if Self::_is_palindrome(substr) {
                    if substr.len() > longest.len() {
                        longest = substr.to_string();
                    }
                }
            }
        }

        longest
    }

    fn _is_palindrome(s: &str) -> bool {
        let is = 0..(s.len()/2);
        let js = (s.len() - 1)..(s.len()/2);

        for (i, j) in is.zip(js) {
            if s.chars().nth(i).unwrap() != s.chars().nth(j).unwrap() {
                return false;
            }
        }

        true
    }
}

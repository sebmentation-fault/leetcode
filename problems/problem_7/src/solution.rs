pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut s = x.to_string();
        let n = s.chars().nth(0);

        if n.is_none() {
            return 0;
        }

        let n = n.unwrap();
        let mut neg = false;

        if n.eq(&'-') {
            neg = true;
            if s.len() > 1 {
                s = s[1..s.len()].to_string();
            } else {
                return 0;
            }
        }

        let rev: String = s.chars().rev().collect();

        let r = i32::from_str_radix(&rev, 10);

        if r.is_err() {
            eprintln!("[Error] failed to parse {:?} into an integer.", &rev);
            return 0;
        }

        let r = r.unwrap();

        if neg {
            return -r;
        }

        r

    }
}

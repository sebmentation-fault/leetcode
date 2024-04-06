pub struct Solution;

impl Solution {
    /// Not my solution, but I translated a C++ version of
    /// the solution into Rust
    pub fn convert(s: String, num_rows: i32) -> String {

        if num_rows <= 1 {
            return s;
        }

        let mut v: Vec<String> = Vec::new();
        let mut j: usize = 0;
        let mut dir = -1;

        for i in 0..s.len() {
            if j as i32 == num_rows - 1 || j == 0 {
                dir *= -1;
            }

            let c = s.chars().nth(i).unwrap();
            let cs = &format!("{:?}", c);

            if j == v.len() {
                v.push(cs.to_string())
            } else {
                v[j] += cs;
            }

            if dir == 1 {
                j += 1;
            } else {
                j -= 1;
            }

        }

        let mut res = String::new();

        for c in v {
            res += &c;
        }

        res.replace("'", "")
    }
}

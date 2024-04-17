pub struct Solution;

impl Solution {
    /// Not my solution
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.trim().chars(); // Trim whitespace and create an iterator
        let mut result = 0i32;
        let mut sign = 1i32;

        // Check if there is a sign character
        let first_char = chars.next();
        if let Some(c) = first_char {
            if c == '-' {
                sign = -1;
            } else if c != '+' {
                if c.is_digit(10) {
                    result = c.to_digit(10).unwrap() as i32;
                } else {
                    return 0; // First non-whitespace character is not a digit or sign
                }
            }
        }

        // Process the rest of the characters
        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as i32;
                // Check for overflow/underflow scenarios
                if sign == 1 && (i32::MAX - digit) / 10 < result {
                    return i32::MAX;
                } else if sign == -1 && (i32::MIN + digit) / 10 > -result {
                    return i32::MIN;
                }
                result = result * 10 + digit;
            } else {
                break; // Stop at the first non-digit character
            }
        }

        result * sign
    }
}

struct _MySolution;

impl _MySolution {
    pub fn _my_atoi(s: String) -> i32 {
        let mut i = 0; // start
        let mut j = 0; // end
        let mut has_num = false;

        for k in 0..s.len() {

            let c = s.chars().nth(k);

            if c.is_none() {
                return 0;
            }

            if c.unwrap().is_digit(10) {
                has_num = true;
                i = k;
                break;
            }

        }

        for k in i+1..s.len() {

            let c = s.chars().nth(k);

            if c.is_none() {
                j = k - 1;
            }

            if !c.unwrap().is_digit(10) {
                j = k - 1;
                break;
            }
        }

        if !has_num {
            return 0;
        }

        let substr = &s[i..j];

        let mut r = 0;

        for k in 0..substr.len() {

            let c = substr.chars().nth(k).unwrap();

            if !c.is_numeric() {
                panic!("Integer substring is not made up of numbers");
            }

            let x = c as i32 - 48;
            let m: i32 = substr.len() as i32 - k as i32;

            r += x * (10^m);
        }

        r
    }

}

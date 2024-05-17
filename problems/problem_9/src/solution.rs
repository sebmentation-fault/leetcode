pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let length = Self::count_digits(x);
        println!("number: {}, length {}", &x, &length);

        for i in 0..((length) / 2) {
            let left = Self::get_digit_at(x, i);
            let right = Self::get_digit_at(x, length - i - 1);
            if left != right {
                println!("false because {} (idx {}) not equal to {} (idx {})", left, i, right, length - i - 1);
                return false;
            }
        }

        return true;
    }

    fn count_digits(x: i32) -> i32 {
        if x < 0 {
            return Self::count_digits(-x);
        } else if x < 10 {
            return 1;
        }

        return 1 + Self::count_digits(x / 10);
    }

    fn get_digit_at(mut x: i32, index: i32) -> i32 {
        if index < 0 {
            panic!("youre asking for something impossible");
        }

        for _ in 0..index {
            x = x / 10;
        }

        return x % 10;
    }
}

// WORKING OUT (for without converting to string)
//
//       121 (true)
//
//       first and last
//       (1)21 / 10 / 10 => 1
//       12(1) mod 10 => 1
//
//       length calculated by how many times / 10 to get to < 10

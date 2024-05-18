pub struct Solution;

#[derive(Debug, Clone)]
enum RegEx {
    Star(Box<RegEx>),
    Dot,
    Char(char),
}

enum ParseError {
    InvalidPattern,
    NotSupported,
    EOP, // End of patterns
}


impl Solution {

    /// Pop the first pattern availabile from the input `patterns`
    ///
    /// If no pattern could be found, return an Error.
    ///
    /// Otherwise, return a RegEx, and the mutated string
    ///
    /// TODO: add any character not just a b c
    fn pop_pattern(patterns: &str) -> Result<(RegEx, &str), ParseError> {

        if patterns.len() == 0 {
            return Err(ParseError::EOP);
        }

        return match patterns {
            ".*" => Ok((RegEx::Star(Box::new(RegEx::Dot)), &patterns[2..])),
            "a*" => Ok((RegEx::Star(Box::new(RegEx::Char('a'))), &patterns[2..])),
            "b*" => Ok((RegEx::Star(Box::new(RegEx::Char('b'))), &patterns[2..])),
            "a" => Ok((RegEx::Char('a'), &patterns[1..])),
            "b" => Ok((RegEx::Char('b'), &patterns[1..])),
            "." => Ok((RegEx::Dot, &patterns[1..])),
            _ => Err(ParseError::InvalidPattern),
        };
    }

    /// Apply RegEx patterns onto a string slice, returning the consumed string slice.
    ///
    /// Returns a ParseError if not possble.
    fn apply_pattern(s: &str, p: Vec<RegEx>) -> Result<&str, ParseError> {

        if p.len() == 0 {
            return Ok(s);
        }

        // Parse (single) char
        if let Some(RegEx::Char(c)) = p.first() {
            if let Some(cc) = s.chars().nth(0) {
                if  *c == cc {
                    return Self::apply_pattern(&s[1..], p[1..].to_vec());
                } else {
                    return Err(ParseError::InvalidPattern);
                }
            } else {
                return Err(ParseError::InvalidPattern);
            }
        }

        // Parse dot (anything)
        // then try to continue the pattern parsing
        if let Some(RegEx::Dot) = p.first() {
            if let Some(_) = s.chars().nth(0) {
                return Self::apply_pattern(&s[1..], p[1..].to_vec());
            } else {
                return Err(ParseError::InvalidPattern);
            }
        }

        // Parse star (zero or more chars)
        // (try 0, then recurse)
        // (if fails, 1, then recurse)
        // until either no longer on valid char OR x too long
        if let Some(RegEx::Star(boxed_regex)) = p.first() {

            let regex: RegEx = (*boxed_regex).as_ref().clone();

            let _ = regex;

            for i in 0..s.len() {
                let ss = &s[i..];

                if let Ok(result) = Self::apply_pattern(ss, p[1..].to_vec()) {
                    return Ok(result);
                }
            }

            return Err(ParseError::NotSupported);
        }

        return Err(ParseError::NotSupported);
    }

    pub fn is_match(s: String, p: String) -> bool {
        let mut pp: &str = &p;
        let mut patterns = vec![];

        // Continuously pop all the available patterns
        // panic if pattern is bad
        loop {
            match Self::pop_pattern(&pp) {
                Ok((re, ppp)) => {
                    patterns.push(re);
                    pp = ppp;
                },
                Err(ParseError::EOP) => {
                    break;
                },
                _ => {
                    panic!("not supported functionality :o")
                },
            }
        }

        println!("the patterns list: {:?}", patterns);

        match Self::apply_pattern(&s, patterns) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

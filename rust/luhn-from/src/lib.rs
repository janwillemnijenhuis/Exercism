use std::fmt;
use std::fmt::{Debug, Formatter};

pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let result = self.code.trim();
        if result.len() > 1 {
            let mut sum = 0;
            let mut even = false;
            for c in result.chars().rev() {
                if c.is_digit(10) || c.is_whitespace() {
                    match c.to_digit(10) {
                        None => {},
                        Some(value) => {
                            if even {
                                let v = if value * 2 <= 9 { value * 2 } else { value * 2 - 9 };
                                sum += v;
                            } else {
                                sum += value;
                            }
                            even = !even;
                        }
                    };
                } else {
                    return false;
                }
            }
            if sum % 10 == 0 {
                return true;
            }
        }
        return false;
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
///
impl <T> From<T> for Luhn where T: ToString {
    fn from(input: T) -> Self {
        Luhn {code: input.to_string()}
    }
}

// impl PartialEq for Luhn {
//     fn eq(&self, other: &Self) -> bool {
//         self.code == other.code
//     }
//
//     fn ne(&self, other: &Self) -> bool {
//         !self.eq(other)
//     }
// }
pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl <T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let temp = self.to_string();
        let result = temp.trim();
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

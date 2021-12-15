/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let result = code.trim();
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

fn main() {
    let test = "4539 3195 0343 6467";
    println!("{}",is_valid(&test));
}

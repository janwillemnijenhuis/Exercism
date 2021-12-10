
fn main() {
    let input = calculator_input("10");
    evaluate(&input).unwrap();
}

fn calculator_input(s: &str) -> Vec<CalculatorInput> {
    s.split_whitespace()
        .map(|s| match s {
            "+" => CalculatorInput::Add,
            "-" => CalculatorInput::Subtract,
            "*" => CalculatorInput::Multiply,
            "/" => CalculatorInput::Divide,
            n => CalculatorInput::Value(n.parse().unwrap()),
        })
        .collect()
}

pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    let mut i = 0;
    while i < inputs.len() {
        match inputs[i] {
            CalculatorInput::Add => {
                if stack.len() >= 2 {
                    let mut val1 = stack.pop().unwrap();
                    let mut val2 = stack.pop().unwrap();
                    stack.push(val1 + val2)
                } else {
                    return None;
                }
            },
            CalculatorInput::Subtract => {
                if stack.len() >= 2 {
                    let mut val1 = stack.pop().unwrap();
                    let mut val2 = stack.pop().unwrap();
                    stack.push(val2 - val1)
                } else {
                    return None;
                }
            },
            CalculatorInput::Multiply => {
                if stack.len() >= 2 {
                    let mut val1 = stack.pop().unwrap();
                    let mut val2 = stack.pop().unwrap();
                    stack.push(val2 * val1)
                } else {
                    return None;
                }
            },
            CalculatorInput::Divide => {
                if stack.len() >= 2 {
                    let mut val1 = stack.pop().unwrap();
                    let mut val2 = stack.pop().unwrap();
                    stack.push(val2 / val1)
                } else {
                    return None;
                }
            },
            CalculatorInput::Value(value) => stack.push(value),
        };
            i += 1;
        }
    if stack.len() != 1 {
        return None;
    } else {
        return Some(stack[0]);
    }
}
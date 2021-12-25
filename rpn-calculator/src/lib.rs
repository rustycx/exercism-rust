#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use crate::CalculatorInput::*;
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() == 0 {
        return None;
    }
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            &CalculatorInput::Value(v) => stack.push(v),
            _ => {
                let (b, a) = (stack.pop()?, stack.pop()?);
                match input {
                    Add => stack.push(a + b),
                    Multiply => stack.push(a * b),
                    Subtract => stack.push(a - b),
                    Divide => stack.push(a / b),
                    _ => unreachable!(),
                }
            }
        }
    }
    stack.pop().and_then(|x| if stack.is_empty() { Some(x) } else { None })
}

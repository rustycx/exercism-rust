#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

use crate::CalculatorInput::*;
pub fn evaluate0(inputs: &[CalculatorInput]) -> Option<i32> {
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
    // stack.pop().and_then(|x| if stack.is_empty() { Some(x) } else { None })
    // stack // .pop() // .map_or_else(|| None, |x| if stack.is_empty() { Some(x) } else { None })
    stack
        .pop()
        .map_or(None, |x| if stack.is_empty() { Some(x) } else { None })
}

// https://exercism.org/tracks/rust/exercises/rpn-calculator/solutions/insideoutclub
use std::ops::{Add, Div, Mul, Sub};
fn binary_operation(stack: &mut Vec<i32>, f: impl Fn(i32, i32) -> i32) -> Option<i32> {
    stack.pop().and_then(|y| stack.pop().map(|x| f(x, y)))
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    inputs
        .iter()
        .try_fold(vec![], |mut stack, x| {
            match *x {
                Add => binary_operation(&mut stack, i32::add),
                Subtract => binary_operation(&mut stack, i32::sub),
                Multiply => binary_operation(&mut stack, i32::mul),
                Divide => binary_operation(&mut stack, i32::div),
                Value(x) => Some(x),
            }
            .map(|x| {
                stack.push(x);
                stack
            })
        })
        .and_then(|stack| match stack.as_slice() {
            [x] => Some(*x),
            _ => None,
        })
}

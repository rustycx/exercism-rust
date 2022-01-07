use std::collections::VecDeque;

#[rustfmt::skip]
pub fn brackets_are_balanced0(string: &str) -> bool {
    let mut stack = VecDeque::new();
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => stack.push_back(c),
            ')' => if stack.pop_back() != Some('(') {return false},
            ']' => if stack.pop_back() != Some('[') {return false},
            '}' => if stack.pop_back() != Some('{') {return false},
            _ => ()
        }
    }
    stack.is_empty()
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();
    for c in string.chars() {
        match c {
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            ']' | '}' | ')' if stack.pop() != Some(c) => return false,
            _ => (),
        }
    }
    stack.is_empty()
}
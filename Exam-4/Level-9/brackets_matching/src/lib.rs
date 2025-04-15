use std::env;

fn is_correctly_bracketed(expression: &str) -> bool {
    let mut stack = Vec::new();
    
    for ch in expression.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            },
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            },
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            },
            _ => (),
        }
    }
    
    stack.is_empty()
}

use std::collections::VecDeque;

pub fn rpn(expression: &str) {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    if tokens.is_empty() {
        println!("Error");
        return;
    }

    let mut stack: VecDeque<i64> = VecDeque::new();

    for token in tokens {
        if let Ok(num) = token.parse::<i64>() {
            stack.push_back(num);
        } else {
            if stack.len() < 2 {
                println!("Error");
                return;
            }

            let b = stack.pop_back().unwrap();
            let a = stack.pop_back().unwrap();

            match token {
                "+" => stack.push_back(a + b),
                "-" => stack.push_back(a - b),
                "*" => stack.push_back(a * b),
                "/" => {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    stack.push_back(a / b)
                }
                "%" => {
                    if b == 0 {
                        println!("Error");
                        return;
                    }
                    stack.push_back(a % b)
                }
                _ => {
                    println!("Error");
                    return;
                }
            }
        }
    }

    if stack.len() != 1 {
        println!("Error");
    } else {
        println!("{}", stack.back().unwrap());
    }
}
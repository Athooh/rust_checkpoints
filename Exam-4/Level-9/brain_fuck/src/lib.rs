use std::env;
use std::io::{self, Write};

const MEMORY_SIZE: usize = 2048;

pub fn brain_fuck(code: &str) {
    let mut memory = vec![0u8; MEMORY_SIZE];
    let mut pointer: usize = 0;
    let mut code_chars: Vec<char> = code.chars().collect();
    let mut pc: usize = 0;
    let mut loop_stack: Vec<usize> = Vec::new();

    while pc < code_chars.len() {
        match code_chars[pc] {
            '>' => {
                pointer += 1;
                if pointer >= MEMORY_SIZE {
                    pointer = 0; // Wrap around
                }
            },
            '<' => {
                if pointer == 0 {
                    pointer = MEMORY_SIZE - 1; // Wrap around
                } else {
                    pointer -= 1;
                }
            },
            '+' => {
                memory[pointer] = memory[pointer].wrapping_add(1);
            },
            '-' => {
                memory[pointer] = memory[pointer].wrapping_sub(1);
            },
            '.' => {
                print!("{}", memory[pointer] as char);
                io::stdout().flush().unwrap();
            },
            '[' => {
                if memory[pointer] == 0 {
                    let mut loop_count = 1;
                    while loop_count > 0 {
                        pc += 1;
                        if pc >= code_chars.len() {
                            panic!("Mismatched '['");
                        }
                        if code_chars[pc] == '[' {
                            loop_count += 1;
                        } else if code_chars[pc] == ']' {
                            loop_count -= 1;
                        }
                    }
                } else {
                    loop_stack.push(pc);
                }
            },
            ']' => {
                if let Some(loop_start) = loop_stack.pop() {
                    if memory[pointer] != 0 {
                        pc = loop_start - 1; // The -1 because it will be incremented by the loop
                        loop_stack.push(loop_start);
                    }
                } else {
                    panic!("Mismatched ']'");
                }
            },
            _ => {} // Ignore any non-command characters
        }
        pc += 1;
    }
}
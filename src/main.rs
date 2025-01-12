use std::env::args;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("No file was given as an argument!");
        return;
    } else if !Path::new(&args[1]).is_file() {
        println!("The given argument was not a valid file!");
        return;
    }
    let code: &str = &std::fs::read_to_string(&args[1]).expect("Failed to read file!");

    let mut stack: Vec<usize> = Vec::new();
    let mut jump: [usize; 32768] = [0; 32768];

    // generate jump table from code dry run
    for (i, ch) in code.char_indices() {
        if ch == '[' {
            stack.push(i);
        } else if ch == ']' {
            let top_entry = stack.pop().expect("Error: trying to pop off empty stack!");
            jump[i] = top_entry;
            jump[top_entry] = i;
        }
    }

    let mut tape: [u8; 32768] = [0; 32768];
    let mut tape_ptr: usize = 0;
    let mut code_ptr: usize = 0;

    // execute
    let chars: Vec<char> = code.chars().collect::<Vec<char>>();
    while code_ptr < code.len() {
        match chars[code_ptr] {
            '>' => tape_ptr += 1,
            '<' => tape_ptr -= 1,
            '+' => tape[tape_ptr] += 1,
            '-' => tape[tape_ptr] -= 1,
            '.' => print!("{}", tape[tape_ptr] as char),
            ',' => {
                tape[tape_ptr] = io::stdin().lock().bytes().next().unwrap().unwrap_or(0);
            }
            '[' => {
                if tape[tape_ptr] == 0 {
                    code_ptr = jump[code_ptr]
                }
            }
            ']' => {
                if tape[tape_ptr] != 0 {
                    code_ptr = jump[code_ptr]
                }
            }
            _ => (),
        }
        code_ptr += 1;
    }
}

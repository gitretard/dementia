use std::{thread,time,io::{self, Write}};
// Should only have 1 main function due to its simplicity
fn main() {
    let mut cells: Vec<u8> = vec![0; 10];
    let mut ptr: usize = 0;
    let mut stack: Vec<usize> = vec![];
    let mut i:usize = 0;
    let s = include_str!("bf");
    while i < s.chars().count() {
        if let Some(char) = s.chars().nth(i) {
            match char {
                '>' => {
                    if (ptr + 1) >= cells.len() {
                        println!("> at {{{i}}} is out of bounds (above {}), ptr is {ptr}", s.len());
                        return;
                    }
                    ptr += 1
                }
                '<' => {
                    if (ptr) == 0 {
                        println!("< at {i} is out of bounds (below 0), ptr is {ptr}");
                        return;
                    }
                    ptr -= 1
                }
                '-' => {
                    if cells[ptr] == 0 {
                        println!("Subtracting from 0!");
                        return;
                    }
                    cells[ptr] = cells[ptr] - 1
                }
                '+' => {
                    if cells[ptr] + 1 >= 255 {
                        return;
                    }
                    cells[ptr] += 1
                }
                '.' => {
                    print!("{}",cells[ptr] as char);
                    io::stdout().flush().unwrap();
                }
                // Still shitty [ and ] implementation
                '[' => {
                    if cells[ptr] == 0 {
                        while s.chars().nth(i).unwrap() != ']'{
                            i+=1;
                        }
                        i+=1;
                        continue;
                    }
                    stack.push(i)
                }
                ']' => {
                    if cells[ptr] != 0 {
                        i = *stack.last().unwrap();
                    } else {
                        stack.pop().unwrap();
                    }
                }
                _ => { /* ignore */ }
            }
        } else {
            println!("Somehow Some(char) returned is None?");
            return;
        }
        //println!("{:?}, ptr: {}, stack: {:?}, i: {i},char: {}", cells,ptr,stack,s.chars().nth(i).unwrap());
        //thread::sleep(time::Duration::from_millis(300));
        i+=1;
    }
    println!()
}

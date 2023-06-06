use std::{
    collections::HashMap,
    env, fmt,
    io::{self, Read, Write},
    panic, process, thread, time,
};

pub trait pretty_unwrap<T, E> {
    fn pretty_unwrap(self) -> T;
    fn pretty_unwrap_or_else(self, val: T) -> T;
}
// I fucking love generics
impl<T, E> pretty_unwrap<T, E> for Result<T, E>
where
    E: fmt::Display,
{
    fn pretty_unwrap(self) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                // Get loc from call
                let loc = panic::Location::caller();
                println!(
                    "\x1b[31m Err! at {} -> {}:{}\n{}\x1b[m",
                    loc.file(),
                    loc.line(),
                    loc.column(),
                    err
                );
                process::exit(1);
            }
        }
    }
    fn pretty_unwrap_or_else(self, val: T) -> T {
        match self {
            Ok(v) => v,
            Err(err) => {
                println!("\x1b[31m{}\x1b[m", err);
                val
            }
        }
    }
}

// when teh actual fuck did rust just stop allowing arguments?
fn args() -> HashMap<String, String> {
    let args: Vec<String> = env::args().collect();
    let mut map = HashMap::new();
    let mut i = 1;
    while i+1 < args.len() {
        let key = args[i].clone();
        let value = args[i+1].clone();
        map.insert(key, value);
        i += 2;
    }
    map
}
fn main() {
    // A shit load of vars
    let opts = args();
    let mut cell: Vec<u8> = vec![
        0;
        opts.get("-c")
            .unwrap_or(&"2".to_string()) // requires a default value, dodgy as always
            .parse()
            .pretty_unwrap()
    ];
    let mut ptr: usize = 0;
    let mut stack: Vec<usize> = vec![];
    let mut i: usize = 0;
    let s = include_str!("bf");
    while i < s.chars().count() {
        if let Some(char) = s.chars().nth(i) {
            match char {
                '>' => {
                    if (ptr + 1) >= cell.len() {
                        println!(
                            "> at {{{i}}} is out of bounds (above {}), ptr: {ptr}",
                            s.len()
                        );
                        return;
                    }
                    ptr += 1
                }
                '<' => {
                    if (ptr) == 0 {
                        println!("< at {{{i}}} is out of bounds (below 0), ptr: {ptr}");
                        return;
                    }
                    ptr -= 1
                }
                '-' => {
                    if cell[ptr] == 0 {
                        println!("Subtracting from 0!");
                        return;
                    }
                    cell[ptr] = cell[ptr] - 1
                }
                '+' => {
                    if cell[ptr] + 1 >= 255 {
                        println!("Attempting to add to cell[ptr] (Overflow)");
                        return;
                    }
                    cell[ptr] += 1
                }
                '.' => {
                    print!("{}", cell[ptr] as char);
                    io::stdout().flush().unwrap();
                }
                ',' => {
                    let mut buf: [u8; 1] = [0; 1];
                    print!("\ninput: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read(&mut buf).unwrap();
                    cell[ptr] = buf[0];
                }
                // Still shitty [ and ] implementation
                '[' => {
                    if cell[ptr] == 0 {
                        // this will crash if [ ]]
                        while s.chars().nth(i).unwrap() != ']' {
                            i += 1;
                        }
                        i += 1;
                        continue;
                    }
                    stack.push(i)
                }
                ']' => {
                    if cell[ptr] != 0 {
                        i = *stack.last().unwrap();
                    } else {
                        stack.pop().unwrap();
                    }
                }
                _ => { i+=1 ;continue;/* ignore */ }
            }
        } else {
            println!("Somehow Some(char) returned is None?");
            return;
        }
        println!(
            "{:?}, ptr: {}, stack: {:?}, i: {i}, char: {}",
            cell,
            ptr,
            stack,
            s.chars().nth(i).unwrap()
        );
        thread::sleep(time::Duration::from_millis(10));
        i += 1;
    }
    println!()
}

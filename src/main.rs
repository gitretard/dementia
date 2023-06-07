use std::{
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

fn main() {
    // A shit load of vars
    let opts: Vec<String> = env::args().collect();
    let mut cellslen: usize = 2;
    let mut i:usize = 0;
    let mut delay:u64 = 0;
    while i < opts.len(){
        match opts[i].as_str(){
            "-c" => {
                cellslen = opts[i+1].trim().parse().pretty_unwrap();
                i+=1
            }
            "-d" => {
                delay = opts[i+1].trim().parse().pretty_unwrap();
                i+=1
            }
            _ => {}
        }
        i+=1
    }
    let mut cell:Vec<u8> = vec![0;cellslen];
    let mut ptr: usize = 0;
    let mut loop_stack: Vec<usize> = vec![];
    let mut i: usize = 0;
    let s = include_str!("bf");
    while i < s.chars().count() {
        if let Some(char) = s.chars().nth(i) {
            match char {
                '>' => {
                    if (ptr + 1) >= cell.len() {
                        println!(
                            "'>' at {{{i}}} is out of bounds (above {}), ptr: {ptr}\nSetting ptr to 0",
                            cell.len()
                        );
                        ptr = 0;
                    }
                    ptr += 1
                }
                '<' => {
                    if (ptr) == 0 {
                        println!("'<' at {{{i}}} is out of bounds (below 0), ptr: {ptr}\nSetting ptr to {}",cell.len()-1);
                        ptr = s.len()-1;
                        return;
                    }
                    ptr -= 1
                }
                '-' => {
                    if cell[ptr] == 0 {
                        println!("Attempting subtract from cell[{ptr}] (Underflow)");
                        return;
                    }
                    cell[ptr] = cell[ptr] - 1
                }
                '+' => {
                    if cell[ptr] + 1 >= 255 {
                        println!("Attempting to add to cell[{ptr}] (Overflow)");
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
                    loop_stack.push(i)
                }
                ']' => {
                    if loop_stack.is_empty() {
                        println!("You have a ']' without a matching '[' at {{{i}}}");
                        return;
                    } else if cell[ptr] != 0 {
                        i = *loop_stack.last().unwrap(); // Dont pop. just read
                    } else {
                        loop_stack.pop().unwrap();
                    }
                }
                _ => {
                    i += 1;
                    continue; /* ignore */
                }
            }
        } else {
            println!("Somehow Some(char) returned is None?");
            return;
        }
        println!(
            "{:?}, ptr: {}, loop_stack: {:?}, i: {i}, char: {}",
            cell,
            ptr,
            loop_stack,
            s.chars().nth(i).unwrap()
        );
        if delay != 0{
            thread::sleep(time::Duration::from_millis(delay));
        }
        i += 1;
    }
    println!()
}

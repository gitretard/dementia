use std::{
    env, fmt, fs,
    io::{self, Read, Write},
    process, thread, time,
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
                println!("\x1b[31m{}\x1b[m", err);
                process::exit(1);
            }
        }
    }
    fn pretty_unwrap_or_else(self, val: T) -> T {
        match self {
            Ok(v) => v,
            Err(err) => {
                println!("\x1b[33m{}\x1b[m", err);
                val
            }
        }
    }
}

// Yeah i know my code suck. I will improve
fn main() {
    // A shit load of vars. Definitely runs on my gaming rig
    let opts: Vec<String> = env::args().collect();
    for i in opts.iter() {
        if i == "-h" || i == "-help" {
            // dunno how to format
            println!("dementia [OPTIONS] TARGET | cargo run -- [OPTIONS] TARGET\nTARGET Must be the last argument\n    -cells [LENGTH]          mount of cells\n    -delay [DELAY]           delay in millisecs\n    -h | -help                help\n    -debug          debug output\nEXAMPLE: dementia -cells 10 -debug -delay 2000 brainfuck.bf");
            return;
        }
    }
    let mut cellslen: usize = 2;
    let mut k: usize = 0;
    let mut delay: u64 = 0;
    let mut dbg: bool = false;
    while k < opts.len() {
        match opts[k].as_str() {
            "-cells" => {
                cellslen = opts[k + 1].trim().parse().pretty_unwrap();
                k += 1
            }
            "-delay" => {
                delay = opts[k + 1].trim().parse().pretty_unwrap();
                k += 1
            }
            "-debug" => {
                dbg = true;
            }
            _ => {
                if k != opts.len() - 1 && k != 0 {
                    println!("Unknown argument: {}", opts[k])
                }
            }
        }
        k += 1
    }
    let mut cell: Vec<u8> = vec![0; cellslen];
    let mut ptr: usize = 0;
    let mut loop_stack: Vec<usize> = vec![];
    let mut ip: usize = 0;
    // Probably angered a few stack evangelists?/extremist?/heap haters?/wtf nvm
    let s = fs::read_to_string(opts.last().unwrap()).pretty_unwrap();
    let ilen = s.chars().count();
    let mut t = s.chars(); // what the actual fuck is wrong with you
    while ip < ilen {
        if let Some(char) = t.nth(ip) {
            println!("{char}");
            match char {
                '>' => {
                    if (ptr + 1) >= cell.len() {
                        println!(
                            "\n'>' at {{{ip}}} is out of bounds (above {}), ptr: {ptr}",
                            cell.len()
                        );
                        return;
                    }
                    ptr += 1
                }
                '<' => {
                    if (ptr) == 0 {
                        println!("\n'<' at {{{ip}}} is out of bounds (below 0), ptr: {ptr}");
                        return;
                    }
                    ptr -= 1
                }
                '-' => {
                    if cell[ptr] == 0 {
                        println!(
                            "\n'-' at {{{ip}}}. Attempting subtract from cell[{ptr}] (Underflow)"
                        );
                        cell[ptr] = u8::MAX; // Underflow to 255. dunno if its right
                    } else {
                    }
                    cell[ptr] = cell[ptr] - 1
                }
                '+' => {
                    if cell[ptr] + 1 >= 255 {
                        println!("\n'+' at {{{ip}}}. Attempting to add to cell[{ptr}] (Overflow)");
                        cell[ptr] = u8::MIN
                    } else {
                        cell[ptr] += 1
                    }
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
                        while t.nth(ip).unwrap() != ']' {
                            ip += 1;
                        }
                        ip += 1;
                        continue;
                    }
                    loop_stack.push(ip)
                }
                ']' => {
                    if loop_stack.is_empty() {
                        println!("\nYou have a ']' without a matching '[' at {{{ip}}}");
                        return;
                    } else if cell[ptr] != 0 {
                        ip = *loop_stack.last().unwrap(); // Dont pop. just read
                    } else {
                        loop_stack.pop().unwrap();
                    }
                }
                ';' => {
                    // A comment
                    println!("!{}",t.nth(ip).unwrap());
                    while t.nth(ip).unwrap() != '\n' {
                        println!("{}",t.nth(ip).unwrap());
                        if ip + 1 >= ilen {
                            return;
                        }
                        ip += 1
                    }
                }
                _ => {
                    ip += 1;
                    continue; /* ignore */
                }
            }
        } else {
            println!("Somehow Some(char) returned is None?");
            return;
        }
        if dbg {
            println!(
                "{:?}, ptr: {}, loop_stack: {:?}, i: {ip}, char: {}",
                cell,
                ptr,
                loop_stack,
                s.chars().nth(ip).unwrap()
            );
        }
        if delay != 0 {
            thread::sleep(time::Duration::from_millis(delay));
        }
        ip += 1;
    }
    println!()
}

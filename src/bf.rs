use std::{
    env,
    fmt,
    fs, io,
    io::{Read, Write},
    process, thread, time,
};
pub trait PrettyUnwrap<T, E> {
    fn pretty_unwrap(self) -> T;
    fn pretty_unwrap_or_else(self, val: T) -> T;
}
// I fucking love generics
impl<T, E> PrettyUnwrap<T, E> for Result<T, E>
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
pub fn run() {
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
    let s: String = fs::read_to_string(opts.last().unwrap()).pretty_unwrap();
    let t: Vec<char> = s.chars().collect::<Vec<_>>();
    let ilen: usize = t.clone().len();
    while ip < ilen {
        match t[ip] {
            '>' => {
                if (ptr + 1) >= cellslen {
                    println!(
                        "\n'>' at {{{ip}}} is out of bounds (above {}), ptr: {ptr}",
                        cellslen
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
                    println!("\n'-' at {{{ip}}}. Attempting subtract from cell[{ptr}] (Underflow)");
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
                if dbg {
                    print!("\ninput: ");
                }
                io::stdout().flush().unwrap();
                io::stdin().read(&mut buf).unwrap();
                cell[ptr] = buf[0];
            }
            // Still shitty [ and ] implementation
            '[' => {
                if cell[ptr] == 0 {
                    while t[ip] != ']' {
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
            // I mean this destroys the whole purpose of brainfuck (of being simple with only 8 commands). but who cares
            ';' => {
                // A comment
                while t[ip] != '\n' {
                    if ip + 1 >= ilen {
                        return;
                    }
                    ip += 1
                }
            }
            '*' => {
                // Bomb
                cell[ptr] = 0
            }
            '_' => {
                // extended print
                // Prints cell[ptr..cell[ptr]]
                let len = cell[ptr] as usize;
                if cell[ptr] == 0 {
                    println!("Attempting to multi-cell print with 0 cells. ip: {ip}, ptr: {ptr} ");
                    return;
                }
                if (ptr + len) >= cellslen {
                    println!("Attempting to multi-cell print. ip: {ip}, ptr: {ptr}, len {len}. ptr + len >= cellslen: {cellslen}");
                    return;
                }
                let mut tmpptr = ptr.clone();
                tmpptr+=1;
                while tmpptr != len {
                    print!("{}", cell[tmpptr] as char);
                    io::stdout().flush().unwrap();
                    tmpptr += 1;
                }
            }
            '/' => {
                // extended input
                // get buffer size from current cell (max 255 ofc)
                let size = cell[ptr] as usize;
                let mut ibuf: Vec<u8> = vec![0; size];
                if size >= cellslen{
                    println!("Attempting to multi-cell read. ip: {ip}, ptr: {ptr}, size: {size}. ptr + size >= cellslen: {cellslen}");
                    return;
                }
                if dbg {
                    print!("\nInput: ");
                    io::stdout().flush().unwrap();
                }
                io::stdin().read(&mut ibuf).unwrap();
                let mut i = 0;
                let ptr = ptr + 1;
                while i < size {
                    cell[ptr + i] = ibuf[i];
                    i+=1
                }
            }
            '~' => {
                // Reset ptr
                ptr = 0
            }
            _ => {
                ip += 1;
                println!("Unknown Char");
                continue; // ignore
            }
        }
        if dbg {
            println!(
                "{:?}, ptr: {}, loop_stack: {:?}, ip: {ip}, char: {}",
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
}

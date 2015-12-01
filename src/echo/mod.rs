extern crate getopts;
use std::process::exit;
use getopts::Options;
use std::io::Write;


#[path = "../utils/mod.rs"]
#[macro_use]
mod utils;

#[allow(dead_code)]
static NAME: &'static str = "echo";
static VERSION: &'static str = "1.0.0";

pub fn mmain(args: &Vec<String>) -> i32 {
    let mut opts = Options::new();

    opts.optflag("n", "", "do not output the trailing newline");
    opts.optflag("e", "", "enable interpretation of backslash escapes");
    opts.optflag("E", "", "disable interpretation of backslash escapes (default)");
    opts.optflag("h", "help", "display this help and exit");
    opts.optflag("V", "version", "output version information and exit");

    let matches = match opts.parse(&args[1..]){
        Ok(m) => m,
        Err(f) => crash!("invalid options\n{}",f)
    };

    if matches.opt_present("help"){
        print_usage(opts);
        return 0;
    }

    if matches.opt_present("version"){
        print_version();
        return 0;
    }

    let input = if !matches.free.is_empty(){
        matches.free.join(" ")
    }
    else{
        "".to_string()
    };


    let output = if matches.opt_present("e"){
        escape(input)
    }
    else{
        input
    };

    print!("{}", output);

    if !matches.opt_present("n"){
        println!("");
    };

    0
}

fn escape(input: String) -> String {
    let mut prev_was_slash = false;
    let mut iter = input.chars().enumerate();
    let mut output = "".to_string();
    while let Some((index, c)) = iter.next() {
        if !prev_was_slash {
            if c != '\\' {
                output.push(c);
            }
            else {
                prev_was_slash = true;
            }
        }
        else {
            prev_was_slash = false;
            match c {
                '\\' => output.push_str("\\"),
                'a'  => output.push_str("\0x07"),
                'b' => output.push_str("\x08"),
                'c' => break,
                'e' => output.push_str("\x1B"),
                'f' => output.push_str("\x0C"),
                'n' => output.push_str("\n"),
                'r' => output.push_str("\r"),
                't' => output.push_str("\t"),
                'v' => output.push_str("\x0B"),
                'x' => {
                    let (c, num_char_used ) = convert_str(input.as_bytes(), index + 1, 16);
                    if num_char_used == 0 {
                        output.push_str("\\x");
                    }
                    else{
                        output.push(c);
                        for _ in 0 .. num_char_used {
                            iter.next();
                        }
                    }
                },
                ch  => {output.push_str("\\"); output.push(ch)},
            };
        }
    }
    output
}

fn isodigit(c : char) -> bool{
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => true,
        _ => false,
    }
}

fn ishdigit(c: char) -> bool{
    match c {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' |
        '8' | '9' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' => true,
        _ => false
    }
}

fn convert_str(string: &[u8], index: usize, base: u32) -> (char, usize) {
    let (max_digits, is_legal_digit) : (usize, fn(char) -> bool) = match base {
        8 => (3, isodigit),
        16 => (2, ishdigit),
        _ => panic!(),
    };

    let mut bytes : String = "".to_string();
    for offset in 0 .. max_digits {
        if string.len() <= index + offset {
            break;
        }
        let c = string[index + offset] as char;
        if is_legal_digit(c) {
            bytes.push(c);
        }
        else {
            break;
        }
    }

    if bytes.len() == 0 {
        (' ', 0)
    }
    else{
        (to_char(&bytes, base), bytes.len())
    }
}

fn to_char(bytes: &str, base: u32) -> char {
    usize::from_str_radix(bytes,base).unwrap() as u8 as char
}

fn print_version(){
    println!("{} {}", NAME, VERSION);
}

fn print_usage(opts: Options){
    let msg = format!("{0} {1} - display a line of text

Usage:
  {0} [SHORT-OPTION]... [STRING]...
  {0} LONG-OPTION

Echo the STRING(s) to standard output.
If -e is in effect, the following sequences are recognized:

\\\\      backslash
\\a      alert (BEL)
\\b      backspace
\\c      produce no further output
\\e      escape
\\f      form feed
\\n      new line
\\r      carriage return
\\t      horizontal tab
\\v      vertical tab
\\0NNN   byte with octal value NNN (1 to 3 digits)
\\xHH    byte with hexadecimal value HH (1 to 2 digits)", NAME, VERSION);

    print!("{}", opts.usage(&msg));
}


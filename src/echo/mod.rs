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
                output = output + &(c.to_string());
            }
            else {
                prev_was_slash = true;
            }
        }
        else {
            prev_was_slash = false;
            let new_str : &str =
                match c {
                    '\\' => "\\",
                    'a'  => "\0x07",
                    'b' => "\x08",
                    'c' => break,
                    'e' => "\x1B",
                    'f' => "\x0C",
                    'n' => "\n",
                    'r' => "\r",
                    't' => "\t",
                    'v' => "\x0B",
                    ch  => &("\\".to_string() + &(ch.to_string())),
                };
            output = output + new_str
        }
    }
    output
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


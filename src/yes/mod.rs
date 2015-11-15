use getopts::Options;
use std::process::exit;
use std::io::Write;

static NAME: &'static str = "yes";
static VERSION: &'static str = "0.0.1";

#[path = "../utils/mod.rs"]
#[macro_use]
mod utils;

pub fn mmain(args: &Vec<String>){
    let mut opts = Options::new();

    opts.optflag("h","help", "display this help and exit");
    opts.optflag("V","version", "output version information and exit");

    let matches = match opts.parse(&args[1..]){
        Ok(m) => m,
        Err(f) => crash!("invalid options\n{}",f)
    };

    if matches.opt_present("version"){
        println!("{} {}", NAME, VERSION);
        exit(0);
    }

    if matches.opt_present("help"){
        print_usage(opts);
        exit(0);
    }

    let string = if matches.free.is_empty(){
        "y".to_string()
    }
    else {
        matches.free.join(" ")
    };

    exec(&string[..]);

    exit(0);
}

fn exec(string: &str) {
    loop {
        println!("{}", string);
    }
}

fn print_usage(opts: Options){
        println!("{} {}", NAME, VERSION);
        println!("");
        println!("Usage:");
        println!("  {0} [STRING]... [OPTION]...", NAME);
        println!("");
        print!("{}", opts.usage("Repeatedly output a line with all specified STRING(s), or 'y'."));
}

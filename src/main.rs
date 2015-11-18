extern crate getopts;
use std::env;
use std::path::Path;
use std::process::exit;
use std::collections::HashMap;

mod yes;

fn main(){
    let args: Vec<String> = env::args().collect();
    exit(dispatch(args));
}


fn dispatch(mut args: Vec<String>) -> i32 {
    // program list
    let mut dispatch_table = HashMap::new();
    dispatch_table.insert("yes", yes::mmain);

    if args.len() == 0 {
        print_usage();
        return 255;
    }
    let path_string =args[0].to_string();
    let path = Path::new(&path_string);
    let command = match path.file_name() {
        Some(file) => file.to_str(),
        _ => panic!("Shouldn't happen!"),
    };

    let command = match command {
        Some(cmd) => cmd,
        _ => panic!("Shouldn't happen!"),
    };

    if command == "busybox" {
        args.remove(0);

        return dispatch(args);
    }

    match dispatch_table.get(command){
        Some(f) => f(&args),
        None => {print_usage(); exit(255);}
    }
}

fn print_usage(){
    println!("busybox command");
    println!("or");
    println!("command");
    println!("");
    println!("Available commands:");
    println!("yes");
}

extern crate getopts;
use std::env;
use std::path::Path;
use std::process::exit;
use std::collections::HashMap;

mod yes;
mod echo;

fn main(){
    let args: Vec<String> = env::args().collect();
    exit(dispatch(args));
}


fn dispatch(mut args: Vec<String>) -> i32 {
    // program list
    let mut dispatch_table: HashMap<&'static str, Box<FnMut(&Vec<String>) -> i32>> = HashMap::new();
    dispatch_table.insert("yes", Box::new(yes::mmain));
    dispatch_table.insert("echo", Box::new(echo::mmain));

    let mut program_list: Vec<String> = vec![];
    for key in dispatch_table.keys(){
        program_list.push(key.to_string());
    }

    if args.len() == 0 {
        print_usage(program_list);
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

    match dispatch_table.get_mut(command){
        Some(mut f) => f(&args),
        None => {print_usage(program_list); exit(255);}
    }
}

fn print_usage(program_list : Vec<String>){
    println!("busybox command");
    println!("or");
    println!("ln busybox command");
    println!("./command");
    println!("");
    println!("Available commands:");
    for key in program_list {
        println!("{}",key);
    }
}

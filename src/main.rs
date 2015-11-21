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

    //to_string() 调用的是 std::fmt::Display
    //clone() 用的是 std::clone::Clone
    //to_owned() 用的是 std::convert::ToOwned
    //到底有何区别
    let program_list: Vec<String> = dispatch_table.keys().map(|s| (*s).to_owned()).collect();

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

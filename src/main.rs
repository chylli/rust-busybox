extern crate getopts;

mod yes;
use std::env;
use std::path::Path;
use std::process::exit;

fn main(){
    let args: Vec<String> = env::args().collect();
    dispatch(args);
}

fn dispatch(mut args: Vec<String>){
    if args.len() == 0 {
        print_usage();
        exit(255);
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

    match AsRef::<str>::as_ref(command){
        "busybox" => {args.remove(0); dispatch(args)},
        "yes" => {yes::mmain(&args)},
        a => {println!("called with {}", a)}
    }

}

fn print_usage(){
    println!("busybox yes");
}

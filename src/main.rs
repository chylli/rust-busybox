extern crate getopts;

mod yes;
use std::env;
use std::path::Path;
use std::process::exit;

fn main(){
    let args: Vec<String> = env::args().collect();
    exit(dispatch(args));
}

fn dispatch(mut args: Vec<String>) -> i32 {
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

    match AsRef::<str>::as_ref(command){
        "busybox" => {args.remove(0); dispatch(args)},
        "yes" => {yes::mmain(&args)},
        a => {println!("called with {}", a); 0}
    }

}

fn print_usage(){
    println!("busybox yes");
}

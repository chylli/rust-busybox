//extern crate getopts;
//extern create libc;
//
//use std::io::Write;
//use std::str::from_utf8;
//#[path = "./util/modrs"]
//#[macro_use]
//mod util;
//
//#[allow(dead_code)]
//static NAME: &'static str = "echo";
//static VERSION: &'static str = "0.0.1";
//
//#[inline(always)]
//struct EchoOPtions {
//    newline: bool,
//    escape: bool
//}
//
//#[inline(always)]
//fn to_char(bytes: &Vec<u8>, base: u32) -> char {
//    usize::from_str_radix(from_utf8(bytes.as_ref()).unwrap(), base).unwrap() as u8 as char
//}
//
//#[inline(always)]
//fn isxdigit(c: u8) -> bool {
//    match c as char {
//        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' |
//        '8' | '9' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' => true,
//        _ => false
//    }
//}
//
//#[inline(always)]
//fn isodigit(c: u8) -> bool {
//    match c as char {
//        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => true,
//        _ => false
//    }
//}

pub fn mmain(args: &Vec<String>) -> i32 {
    println!("echo is called");
    0
}

use std::process::Command;
use std::str;

static PROGRAM: &'static str = "target/debug/busybox";
static COMMAND: &'static str = "echo";

#[test]
fn test_default(){
    let po = Command::new(PROGRAM).arg(COMMAND)
        .output()
        .unwrap_or_else(|err| panic!("{}",err));

    let out = str::from_utf8(&po.stdout[..]).unwrap();
    assert_eq!(out, "\n");
}

#[test]
fn test_no_trailing_newline(){
    let po = Command::new(PROGRAM).arg(COMMAND)
        .arg("-n")
        .arg("hello_world")
        .output()
        .unwrap_or_else(|err| panic!("{}", err));

    let out = str::from_utf8(&po.stdout[..]).unwrap();
    assert_eq!(out, "hello_world");
}

#[test]
fn test_enable_escapes() {
    let po = Command::new(PROGRAM).arg(COMMAND)
        .arg("-e")
        .arg("\\\\\\t\\r")
        .output()
        .unwrap_or_else(|err| panic!("{}", err));

    let out = str::from_utf8(&po.stdout[..]).unwrap();
    assert_eq!(out, "\\\t\r\n");
}

#[test]
fn test_disable_escapes() {
    let po = Command::new(PROGRAM).arg(COMMAND)
        .arg("-E")
        .arg("\\b\\c\\e")
        .output()
        .unwrap_or_else(|err| panic!("{}", err));

    let out = str::from_utf8(&po.stdout[..]).unwrap();
    assert_eq!(out, "\\b\\c\\e\n");
}

#[test]
fn test_escape_hex(){
    let po = Command::new(PROGRAM).arg(COMMAND)
        .arg("-e")
        .arg("\\x70\\0110")
        .output()
        .unwrap_or_else(|err| panic!("{}", err));

    let out = str::from_utf8(&po.stdout[..]).unwrap();
    assert_eq!(out, "pH\n");
}

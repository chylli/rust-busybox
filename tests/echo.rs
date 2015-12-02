use std::process::Command;
use std::str;

static PROGRAM: &'static str = "target/debug/busybox";
static COMMAND: &'static str = "echo";

fn command() -> Command {
    let mut c = Command::new(PROGRAM);
    c.arg(COMMAND);
    c
}
#[test]
fn test_default(){
    let po = command()
        .output()
        .unwrap_or_else(|err| panic!("{}",err));

    let out = str::from_utf8(&po.stdout[..]).unwrap();
    assert_eq!(out, "\n");
}

#[test]
fn test_no_trailing_newline(){
    let po = command()
        .arg("-n")
        .arg("hello_world")
        .output()
        .unwrap_or_else(|err| panic!("{}", err));

    let out = str::from_utf8(&po.stdout[..]).unwrap();
    assert_eq!(out, "hello_world");
}

#[test]
fn test_enable_escapes() {
    let po = command()
        .arg("-e")
        .arg("\\\\\\t\\r")
        .output()
        .unwrap_or_else(|err| panic!("{}", err));

    let out = str::from_utf8(&po.stdout[..]).unwrap();
    assert_eq!(out, "\\\t\r\n");
}

#[test]
fn test_disable_escapes() {
    let po = command()
        .arg("-E")
        .arg("\\b\\c\\e")
        .output()
        .unwrap_or_else(|err| panic!("{}", err));

    let out = str::from_utf8(&po.stdout[..]).unwrap();
    assert_eq!(out, "\\b\\c\\e\n");
}

#[test]
fn test_escape_hex(){
    let po = command()
        .arg("-e")
        .arg("\\x70\\0110")
        .output()
        .unwrap_or_else(|err| panic!("{}", err));

    let out = str::from_utf8(&po.stdout[..]).unwrap();
    assert_eq!(out, "pH\n");
}

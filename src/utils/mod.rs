#[macro_export]
macro_rules! crash(
    ($($arg:tt)*) => ({
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        };
        exit(255)
    })
 );

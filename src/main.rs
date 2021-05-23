use std::fmt;
use std::error::Error;

trait BaseErrorFmt : fmt::Display + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You passed one to fun_needs_zero")
    }
}

trait BaseErrorError : Error {
}

#[derive(Debug)] // Needed for fmt not to complain
struct ErrorNumberIsOne {
    c : &'static str, // Is this correct? I want read-only const string here
}


impl ErrorNumberIsOne {
    fn new() -> ErrorNumberIsOne {
        ErrorNumberIsOne {
            c : "You passed one to fun_needs_zero"
        }
    }
}


impl BaseErrorFmt for ErrorNumberIsOne {
}
impl fmt::Display for ErrorNumberIsOne {}

fn main() {
    println!("Hello, world!");
}

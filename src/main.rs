use std::fmt;
use std::error::Error;

trait BaseError : Error + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You passed one to fun_needs_zero")
    }
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
impl fmt::Display for ErrorNumberIsOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You passed one to fun_needs_zero")
    }
}

impl Error for ErrorNumberIsOne {

}


fn main() {
    println!("Hello, world!");
}

use std::fmt;
use std::error::Error;

#[derive(Debug)] // Needed for fmt not to complain
struct ErrorNumberIsOne {
    c : &'static str,
}

impl ErrorNumberIsOne {
    fn new() -> ErrorNumberIsOne {
        ErrorNumberIsOne {
            c : "You passed one to fun_needs_zero"
        }
    }
}
impl fmt::Display for ErrorNumberIsOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c)
    }
}

impl Error for ErrorNumberIsOne {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

#[derive(Debug)] // Needed for fmt not to complain
struct ErrorNumberIsTwo {
    c : &'static str,
}

impl ErrorNumberIsTwo {
    fn new() -> ErrorNumberIsTwo {
        ErrorNumberIsTwo {
            c : "You passed two to fun_needs_zero"
        }
    }
}
impl fmt::Display for ErrorNumberIsTwo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.c)
    }
}

impl Error for ErrorNumberIsTwo {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

#[derive(Debug)] // Needed for fmt not to complain
struct ErrorNumberIsOther {
    c : &'static str,
    num : u32,
}

impl ErrorNumberIsOther {
    fn new(other : u32) -> ErrorNumberIsOther {
        ErrorNumberIsOther {
            c : "You passed some other number to fun_needs_zero",
            num : other
        }
    }
}
impl fmt::Display for ErrorNumberIsOther {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.c, self.num)
    }
}

impl Error for ErrorNumberIsOther {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

fn fun_needs_zero(number: u32) -> Result<(), Box<dyn Error>> {
    match number {
        0 => Ok(()),
        1 => Err(Box::new(ErrorNumberIsOne::new())),
        2 => Err(Box::new(ErrorNumberIsTwo::new())),
        _ => Err(Box::new(ErrorNumberIsOther::new(number))),
    }
}

fn error_prone_fun() -> Result<(), Box<dyn Error>> {
    let mut line = String::new();
    if let Err(err) = std::io::stdin().read_line(&mut line) {
        return Err(Box::new(err));
    }; // any way to make ? operator work here

    let line: u32 = line.trim().parse()?;
    fun_needs_zero(line)
}

fn main() {
    match error_prone_fun() {
        Ok(_) => println!("error_prone_fun finished ok!"),
        Err(err) => {
            println!("we got an error!");
            if let Some(ev) = err.downcast_ref::<ErrorNumberIsOne>() {
                println!("ErrorNumberIsOne error: {}", ev);
            }
            else if let Some(ev) = err.downcast_ref::<ErrorNumberIsTwo>() {
                println!("ErrorNumberIsTwo error: {}", ev);
            }
            else if let Some(ev) = err.downcast_ref::<ErrorNumberIsOther>() {
                println!("ErrorNumberIsOther error: {}", ev);
            }
            else {
                println!("Other application error: {}", err);
            }
        }
    }
}

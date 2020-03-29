use std::fmt;

#[derive(Debug)]
pub enum Object {
    Number(f64),
    Str(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Object::Number(number) => write!(f, "{}", number),
            Object::Str(string) => write!(f, "{}", string),
        }
    }
}

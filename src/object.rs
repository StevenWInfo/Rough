use std::fmt;

#[derive(Debug)]
pub enum Object {
    Number(f64),
    Str(String),
}

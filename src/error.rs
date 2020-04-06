
/// For functionality regarding handling and displaying
/// errors in the Rough code.
/// Note: this isn't about errors in the interpreter,
/// that is handled differently.

// This might change quite a bit.
// Might have more than one error type.
// Might have a list of messages, errors, etc.
#[derive(Debug, PartialEq)]
pub struct RoughError {
    msg: String,
}

impl RoughError {
    pub fn new(message: String) -> RoughError {
        RoughError {
            msg: message,
        }
    }
}

// Is handling multiple errors in the result better or in the lexer/parser/evaluators respectively?
pub type RoughResult<T> = Result<T, Vec<RoughError>>;

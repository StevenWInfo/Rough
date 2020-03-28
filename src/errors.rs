
/// For functionality regarding handling and displaying
/// errors in the Rough code.
/// Note: this isn't about errors in the interpreter,
/// that is handled differently.

// This might change quite a bit.
// Might have more than one error type.
// Might have a list of messages, errors, etc.
pub struct RoughError {
    msg: String,
}

pub type RoughResult<T> = Result<T, Vec<RoughError>>

use crate::token::TokenType;

// Need a way to use operators like normal functions. E.g. Haskells (+)
// Also, would like a way to use normal functions like operators (for a single instale like
// Haskell).

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Precedence {
    First = 0,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
}

// Haven't implemented anything making this necessary yet (eventually will)
pub fn reserved_precedences(token: &TokenType) -> Option<Precedence> {
    match token {
        _ => None
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OperatorType {
    Prefix,
    Infix,
    Postfix,
}

// Might use this for the evaluator too.
// Could expand for things like lists, but is it really beneficial?
// Could maybe parse expression looking for the next symbol in operator, but meh
#[derive(Debug, PartialEq, Clone)]
pub struct OperatorDefinition {
    pub identifier: String,
    pub op_type: OperatorType,
    pub precedence: Precedence,
}

// There must be some sort of taxonomy of syntactical constructs that organizes things like this.
// Value, operator, lists, etc.
// Actually, I'm pretty sure this is what parser generators and/or combinators are.
// I sort of feel like I'm falling into a trap here.
// I also sort of feel like this is just making a "grammar" data structure.

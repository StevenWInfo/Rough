use crate::token::TokenType;

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
fn precedences(token: &TokenType) -> Option<Precedence> {
    match token {
        _ => None
    }
}

pub enum OperatorType {
    Prefix,
    Infix,
    Postfix,
}

// Might use this for the evaluator too.
// Could expand for things like lists, but is it really beneficial?
// Could maybe parse expression looking for the next symbol in operator, but meh
pub struct OperatorDefinition {
    identifier: String,
    op_type: OperatorType,
    // Could set per parameter, but might be cumbersome.
    short_circuit: bool,
    precedence: Precedence,
}

impl OperatorDefintion {
    pub fn param_num(&self) {
        self.infixes.len() + 1
    }
}

// There must be some sort of taxonomy of syntactical constructs that organizes things like this.
// Value, operator, lists, etc.
// Actually, I'm pretty sure this is what parser generators and/or combinators are.
// I sort of feel like I'm falling into a trap here.
// I also sort of feel like this is just making a "grammar" data structure.

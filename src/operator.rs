

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

// Might use this for the evaluator too.
pub struct OperatorDefinition {
    // Need at least one of these three. Can I type check it?
    prefix: Option<String>,
    infixes: Vec<String>,
    postfix: Option<String>,
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

use crate::operator::OperatorDefinition;

// Should I just add the short circuited things to the language
// rather than setting up a whole system for it?
// Short circuited things: if conditions, other potential conditions,
// boolean operators. The conditions are already handled specially.
// Are there things other than boolean operators that it would be
// useful for? Specified lazy evaluation?
// Or just have it not short circuit initially and add it after
// basic language stuff.

pub enum Expression {
    Ident(String),
    Number(f64),
    Str(String),
    Function(Vec<String>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Prefix(OperatorDefinition, Box<Expression>),
    Infix(Box<Expression>, OperatorDefinition, Box<Expression>),
    Postfix(Box<Expression>, OperatorDefinition),
    // TODO Correct
    IndexMap(Vec<Expression>),
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
}

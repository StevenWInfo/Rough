use crate::operator::OperatorDefinition;

pub enum Expression {
    Ident(String),
    Number(f64),
    Str(String),
    Function(Vec<String>, Box<Expression>),
    Call(Box<Expression, Vec<Expression>),
    Operator(Vec<String>, Vec<Expression>),
    Prefix(OperatorDefinition, Box<Expression>),
    Infix(Box<Expression>, OperatorDefition, Box<Expression>),
    Postfix(Box<Expression>, OperatorDefition),
    IndexMap(Vec<Expression>),
    Operator(OperatorExpression),
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
}

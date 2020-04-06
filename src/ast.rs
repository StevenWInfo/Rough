use crate::operator::OperatorDefinition;

pub struct OperatorExpression {
    operator: OperatorDefinition,
    args: Vec<Expression>,
}

pub enum Expression {
    Ident(String),
    Number(f64),
    Str(String),
    Function(Vec<String>, Box<Expression>),
    Call(Box<Expression, Vec<Expression>),
    Operator(Vec<String>, Vec<Expression>),
    IndexMap(Vec<Expression>),
    Operator(OperatorExpression),
}

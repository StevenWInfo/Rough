use crate::lexer::Lexer;
use crate::operator::{ OperatorDefinition, Precedence };
use crate::error::{ RoughError, RoughResult };
use crate::ast::Expression;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    operators: Vec<OperatorDefinition>
    errors: Vec<RoughError>,
    // Wouldn't be necessary if I could figure out how to return closures
    cur_token: RoughResult<Token>,
}

static empty_early_error = RoughError::new("Source ended before making a valid expression");

impl Parser<'_> {
    pub fn new(mut lex: Lexer, operators: Vec<OperatorDefinition>) -> Parser {
        // This might be a little too "clever"
        let token = match lex.next() {
            Some(token) => Ok(token),
            None => Err(vec![empty_early_error]),
        };

        Parser {
            lexer: lex.filter(|token| !self.ignored(token)).peekable(),
            errors: Vec::new(),
            cur_token: token,
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors
    }

    fn ignored(token: &Token) -> bool {
        match token {
            Token::Comment => true,
            Token::Space => true,
            Token::Tab => true,
            Token::Newline => true,
            _ => false
        }
    }

    pub fn parse_program(&mut self) -> RoughResult<Expression> {
        let exp = self.parse_expression(Precedence::First);

        /* Might enforce this later.
        match self.cur_token {
            Some(_) => RoughResult::new("Only expecting one expression"),
            None => exp
        }
        */

        exp
    }

    fn parse_expression(&mut self, precedence: Precedence) -> RoughResult<Expression> {
        let token = self.cur_token
        let expr = prefix_parse_lookup(&token)?;
    }
}

fn parse_number(parser: &mut Parser) -> RoughResult<Expression> {
    match &parser.cur_token? {
        Token::Number(num) => Ok(Expression::Number(*num)),
        other => Err(vec![format!("Expected Number token, but got {}", other).to_string()]),
    }
}

fn parse_string_literal(parser: &mut Parser) -> RoughResult<Expression> {
    match &parser.cur_token? {
        Token::Str(string) => Ok(Expression::Str(string.to_string())),
        other => Err(vec![format!("Expected Str token, but got {}", other).to_string()]),
    }
}

fn parse_function_parameters(parser: &mut Parser) -> RoughResult<Expression> {
}

fn parse_function_literal(parser: &mut Parser) -> RoughResult<Expression> {
}

fn parse_assign(parser: &mut Parser) -> RoughResult<Expression> {
}

type PrefixParseFn = fn(parser: &mut Parser) -> RoughResult<Expression>;
type InfixParseFn = fn(parser: &mut Parser, left_exp: Expression) -> RoughResult<Expression>;

fn parse_prefix_expression(parser: &mut Parser) -> RoughResult<Expression> {
}

fn parse_infix_expression(parser: &mut Parser, left_exp: Expression) -> RoughResult<Expression> {
    // Calling functions will go in here too.
}

fn prefix_parse_lookup(token: &Token) -> RoughResult<Expression> {
    match token {
        Token::Number(_) => parse_number,
        Token::Str(_) => parse_string_literal,
        Token::Pipe => parse_function,
        Token::Ident(_) => parse_prefix_expression,
        _ => RoughError::new(format!("prefix_parse_lookup doesn't have token {}", token).to_string()),
    }
}

fn infix_parse_lookup(token: &Token) -> Option<Expression> {
    match token {
        Token::Assign => Some(parse_assign),
        Token::Ident(_) => Some(parse_infix_expression),
        _ => None
    }
}

use crate::lexer::Lexer;
use crate::operator::{ OperatorDefinition, Precedence, reserved_precedences, OperatorType };
use crate::error::{ RoughError, RoughResult, new_error };
use crate::ast::Expression;
use crate::token::{ Token, TokenType };
use std::iter::{ Peekable };

pub struct Parser<'a> {
    // I want to have it just be the iterator, but the types for iterators are too annoying to deal
    // with right now. Figure it out later.
    // (What was I using this field for again?)
    tokens: Vec<Token>,
    lexer: Peekable<Lexer<'a>>,
    //lexer: Peekable<Filter<'a>>,
    operators: Vec<OperatorDefinition>,
    errors: Vec<RoughError>,
    // Wouldn't be necessary if I could figure out how to return closures
    cur_token: Option<Token>,
}

static empty_early_error: RoughError = RoughError::new("Source ended before making a valid expression".to_string());

impl Parser<'_> {
    pub fn new(mut lex: Lexer, operators: Vec<OperatorDefinition>) -> Parser {
        // Annoyances made me do this strange dance. Maybe clean up later
        let tokens = lex.collect();

        let parser = Parser {
            lexer: tokens.iter().peekable(),
            tokens: tokens,
            operators: operators,
            errors: lex.errors,
            cur_token: None,
        };

        parser.next();
        parser
    }

    pub fn current_result(&self) -> RoughResult<Token> {
        match self.cur_token {
            Some(token) => Ok(token),
            None => Err(vec![empty_early_error]),
        }
    }

    pub fn next(&mut self) {
        for token in self.lexer {
            if !ignored(token) {
                self.cur_token = token;
                return ();
            }
        }
        self.cur_token = None;
    }

    fn peek(&self) -> Option<Token> {
        while let Some(peek_tok) = self.lexer.peek() {
            if !ignored(peek_tok) {
                break;
            }
            self.lexer.next();
        }

        self.lexer.peek()
    }

    fn next_if_equals(&mut self, expected: TokenType) -> bool {
        let equals = self.peek()
            .map(|token| token.token_type == expected)
            .unwrap_or(false);

        if equals {
            self.next();
            true
        } else {
            false
        }
    }

    fn next_if_equals_result(&mut self, expected: TokenType) -> RoughResult<()> {
        if !self.next_if_equals(expected) {
            new_error(format!("Expected next token to be {} but it was {}", expected, self.current_result()?))
        } else {
            Ok(())
        }
    }

    pub fn get_errors(&self) -> Vec<RoughError> {
        self.errors
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
        let token = self.current_result()?;
        let prefix_parser = prefix_parse_lookup(&token)?;

        let mut exp = prefix_parser(self)?;

        // If it finds another Ident, it should assume a function call
        while self.peek().is_some() && precedence < self.token_precedence(&self.peek().unwrap()) {
            let infix = match infix_parse_lookup(&self.peek().unwrap()) {
                Some(infix_op) => infix_op,
                // Is this ok in this implementation?
                None => return Ok(exp),
            };

            self.next();
            exp = infix(self, exp)?;
        }

        Ok(exp)
    }

    fn token_precedence(&self, token: &Token) -> Precedence {
        if let Some(prec) = reserved_precedences(&token.token_type) {
            return prec;
        }

        // Check given operators
        if let TokenType::Ident(ident) = token.token_type {
            if let Some(op) = self.operators.iter().find(|op| op.identifier == ident) {
                return op.precedence
            }
        }

        Precedence::First
    }
}

fn parse_number(parser: &mut Parser) -> RoughResult<Expression> {
    match &parser.current_result()?.token_type {
        TokenType::Number(num) => Ok(Expression::Number(*num)),
        other => new_error(format!("Expected Number token, but got {}", other).to_string()),
    }
}

fn parse_string_literal(parser: &mut Parser) -> RoughResult<Expression> {
    match &parser.current_result()?.token_type {
        TokenType::Str(string) => Ok(Expression::Str(string.to_string())),
        other => new_error(format!("Expected Str token, but got {}", other).to_string()),
    }
}

fn parse_function_parameters(parser: &mut Parser) -> RoughResult<Vec<String>> {
    let mut params = vec![];

    parser.next();

    match parser.current_result()?.token_type {
        TokenType::Ident(name) => params.push(name.to_string()),
        other => return Err(vec![RoughError::new(format!("Function parameter expected an Ident token but was {}", other))]),
    }

    while parser.peek().map(|tok| tok.token_type) == Some(TokenType::Comma) {
        parser.next();
        parser.next();

        let current = parser.current_result()?;

        if let TokenType::Ident(name) = current.token_type {
            params.push(name.to_string())
        } else {
             return new_error(format!("Function parameter expected an Ident token but was {}", current));
        }
    }

    parser.next_if_equals_result(TokenType::Pipe);

    Ok(params)
}

fn parse_function_literal(parser: &mut Parser) -> RoughResult<Expression> {
    Ok(
        Expression::Function(
            parse_function_parameters(parser)?,
            Box::new(parser.parse_expression(Precedence::First)?)
            )
      )
}

fn parse_if_expression(parser: &mut Parser) -> RoughResult<Expression> {
    let cond = parser.parse_expression(Precedence::First)?;

    let cons = parser.parse_expression(Precedence::First)?;

    match parser.lexer.peek() {
        Some(token) if token.token_type == TokenType::Else => {
            let parsed_else = parser.parse_expression(Precedence::First)?;
            Ok(Expression::If(Box::new(cond), Box::new(cons), Some(Box::new(parsed_else))))
        },
        _ => Ok(Expression::If(Box::new(cond), Box::new(cons), None)),
    }
}

fn parse_grouped_expression(parser: &mut Parser) -> RoughResult<Expression> {
    parser.next();
    let exp = parser.parse_expression(Precedence::First);

    let current = parser.current_result()?;

    parser.next_if_equals_result(TokenType::RParen)?;

    exp
}

// TODO actually implement map part.
fn parse_index_map_literal(parser: &mut Parser) -> RoughResult<Expression> {
    let mut elems: Vec<Expression> = vec![];

    if parser.peek().map(|tok| tok.token_type) == Some(TokenType::RBracket) {
        parser.next();
        return Ok(Expression::IndexMap(elems))
    }

    parser.next();

    elems.push(parser.parse_expression(Precedence::First)?);

    while parser.peek().map(|tok|tok.token_type == TokenType::Comma).unwrap_or(false) {
        parser.next();
        parser.next();

        elems.push(parser.parse_expression(Precedence::First)?);
    }

    parser.next_if_equals_result(TokenType::RBracket)?;

    Ok(Expression::IndexMap(elems))
}

// Might need to figure out function calling here too.
fn parse_prefix_expression(parser: &mut Parser) -> RoughResult<Expression> {
    let op_token: Token = parser.current_result()?;

    let op_start = match op_token.token_type {
        TokenType::Ident(name) => name,
        other => return Err(vec![RoughError::new(format!("Should be an Ident token but got {}. Not sure how it even got here.", other))]),
    };

    let op_def = parser.operators
        .filter(|op| op.op_type == OperatorType::Prefix)
        .find(|op| op.name == op_start);

    parser.next();

    let right_exp = parser.parse_expression(op_def.precedence)?;
    Ok(Expression::Prefix(op_def, Box::new(right_exp)))
}

fn parse_infix_expression(parser: &mut Parser, left_exp: Expression) -> RoughResult<Expression> {
    let op_ident = match parser.current_result()?.token_type {
        TokenType::Ident(op_ident) => op_ident,
        other => return new_error(format!("Expected ident token, got {}", other)),
    };

    let op_def_option = parser.operators
        .iter()
        .filter(|op| op.op_type == OperatorType::Infix)
        .find(|op| op.identifier == op_ident);

    let op_def = match op_def_option {
        Some(op_def) => op_def,
        None => return new_error(format!("Could not find a defined operator that matched {}", op_ident))
    };

    parser.next();

    let right_exp = parser.parse_expression(op_def.precedence)?;

    Ok(Expression::Infix(Box::new(left_exp), *op_def, Box::new(right_exp)))
}

type PrefixParseFn = fn(parser: &mut Parser) -> RoughResult<Expression>;
type InfixParseFn = fn(parser: &mut Parser, left_exp: Expression) -> RoughResult<Expression>;

fn prefix_parse_lookup(token: &Token) -> RoughResult<PrefixParseFn> {
    let func = match token.token_type {
        TokenType::Number(_) => parse_number,
        TokenType::Str(_) => parse_string_literal,
        TokenType::Pipe => parse_function_literal,
        TokenType::Ident(_) => parse_prefix_expression,
        TokenType::If => parse_if_expression,
        TokenType::LParen => parse_grouped_expression,
        TokenType::LBracket => parse_index_map_literal,
        _ => return Err(vec![RoughError::new(format!("prefix_parse_lookup doesn't have token {}", token).to_string())]),
    };

    Ok(func)
}

fn infix_parse_lookup(token: &Token) -> Option<InfixParseFn> {
    match token.token_type {
        TokenType::Ident(_) => Some(parse_infix_expression),
        // TODO Do later. Make a built in function for now and add this in later.
        //Token::LBracket => Some(parse_index_expression),
        _ => None
    }
}

// I suppose not ignoring whitespace might break a lot of code currently.
// Should deal with this sooner rather than later.
fn ignored(token: Token) -> bool {
    match token.token_type {
        TokenType::Comment(_) => true,
        TokenType::Space => true,
        TokenType::Tab => true,
        TokenType::Newline => true,
        _ => false
    }
}

use crate::lexer::Lexer;
use crate::operator::{ OperatorDefinition, Precedence, reserved_precedences, OperatorType };
use crate::error::{ RoughError, RoughResult, new_error };
use crate::ast::Expression;
use crate::token::{ Token, TokenType };
use std::iter::{ Peekable, Filter };

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    //lexer: Peekable<Filter<'a>>,
    operators: Vec<OperatorDefinition>
    errors: Vec<RoughError>,
    // Wouldn't be necessary if I could figure out how to return closures
    cur_token: Option<Token>,
}

static empty_early_error: RoughError = RoughError::new("Source ended before making a valid expression".to_string());

impl Parser<'_> {
    pub fn new(mut lex: Lexer, operators: Vec<OperatorDefinition>) -> Parser {
        let mut starting_errors = Vec::new();
        let mut starting_token = None;

        for result in lex {
            if let Ok(token) = result {
                starting_token = token;
                break;
            } else {
                starting_errors.push(result);
            }
        }

        Parser {
            lexer: lex.peekable(),
            operators: operators,
            errors: starting_errors,
            cur_token: starting_token,
        }
    }

    pub fn current_fail(&self) -> RoughResult<Token> {
        match self.cur_token {
            Some(token) => Ok(token),
            None => Err(vec![empty_early_error]),
        }
    }

    pub fn next(&mut self) {
        self.cur_token = self.lexer.next();
    }

    fn next_if_equals(&mut self, expected: TokenType) -> bool {
        let equals = self.lexer.peek()
            .map(|token| token?.token_type == expected)
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

    pub fn errors(&self) -> Vec<RoughError> {
        let errors = self.lexer.errors.clone();
        errors.append(self.errors.clone());
        errors
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
        while self.lexer.peek().is_some() && precedence < self.token_precedence(&self.peek().unwrap()) {
            let infix = match infix_parse_lookup(self.peek().unwrap()) {
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
        if let Some(prec) = reserved_precedences(token.token_type) {
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
    match &parser.current()?.token_type {
        TokenType::Number(num) => Ok(Expression::Number(*num)),
        other => new_error(format!("Expected Number token, but got {}", other).to_string()),
    }
}

fn parse_string_literal(parser: &mut Parser) -> RoughResult<Expression> {
    match &parser.current()?.token_type {
        TokenType::Str(string) => Ok(Expression::Str(string.to_string())),
        other => new_error(format!("Expected Str token, but got {}", other).to_string()),
    }
}

fn parse_function_parameters(parser: &mut Parser) -> RoughResult<Vec<String>> {
    let mut params = vec![];

    parser.next();

    match parser.current()?.token_type {
        TokenType::Ident(name) => params.push(name.to_string()),
        other => return Err(vec![RoughError::new(format!("Function parameter expected an Ident token but was {}", other))]),
    }

    while parser.peek()?.token_type == TokenType::Comma {
        parser.next();
        parser.next();

        let current = parser.current()?;

        if let TokenType::Ident(name) = current.token_type {
            params.push(name.to_string())
        } else {
             return new_error(format!("Function parameter expected an Ident token but was {}", current)),
        }
    }

    parser.next_if_equals_result(TokenType::Pipe);

    Ok(params)
}

fn parse_function_literal(parser: &mut Parser) -> RoughResult<Expression> {
    Ok(
        Expression::Function(
            parse_function_parameters(parser)?,
            parser.parse_expression(Precedence::First)?
            )
      )
}

fn parse_if_expression(parser: &mut Parser) -> RoughResult<Expression> {
    let cond = parser.parse_expression(Precedence::First)?;

    let cons = parser.parse_expression(Precedence::First)?;

    match parser.lexer.peek() {
        Some(token) if token?.token_type == TokenType::Else => {
            let parsed_else = parser.parse_expression(Precedence::First)?;
            Ok(Expression::If(Box::new(cond), Box::new(cons), Some(Box::new(parsed_else))))
        },
        _ => Ok(Expression::If(Box::new(cond), Box::new(cons), None)),
    }
}

fn parse_grouped_expression(parser: &mut Parser) -> RoughResult<Expression> {
    parser.next();
    let exp = parser.parse_expression(Precedence::First)?;

    let current = parser.current()?;

    match current.token_type {
        TokenType::RParen => parser.next(),
        other => return Err(vec![RoughError::new(format!("Expected right paren at the end of expression, got {}", other))])
    };

    exp
}

// TODO actually implement map part.
fn parse_index_map_literal(parser: &mut Parser) -> RoughResult<Expression> {
    let mut elems: Vec<Expression> = vec![];

    if parser.peek()?.token_type == TokenType::RBracket {
        parser.next();
        return Ok(Expression::IndexMap(elems))
    }

    parser.next();

    elems.push(parser.parse_expression(Precedence::First)?);

    while parser.peek()?.token_type == TokenType::Comma {
        parser.next();
        parser.next();

        elems.push(parser.parse_expression(Precedence::Frist)?);
    }

    parser.next_if_equals_result(TokenType::RBracket)?;

    Ok(Expression::IndexMap(elems))
}

// Might need to figure out function calling here too.
fn parse_prefix_expression(parser: &mut Parser) -> RoughResult<Expression> {
    let op_token: Token = parser.current()?;

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
    let op_token: Token = parser.current()?;

    let op_def = parser.operators
        .filter_map(|op| op.infix)
        .find(|op| op.name == op_token);

    parser.next();

    let right_exp = parser.parse_expression(op_def.precedence);
    
    Ok(Expression::Infix(Box::new(left_exp), op_def, Box::new(right_exp)))
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
fn ignored(token_result: &RoughResult<Token>) -> bool {
    if let Ok(token) = token_result {
        match token.token_type {
            TokenType::Comment(_) => true,
            TokenType::Space => true,
            TokenType::Tab => true,
            TokenType::Newline => true,
            _ => false
        }
    } else {
        false
    }
}

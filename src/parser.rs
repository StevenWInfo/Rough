use crate::lexer::Lexer;
use crate::operator::{ OperatorDefinition, Precedence };
use crate::error::{ RoughError, RoughResult };
use crate::ast::Expression;
use crate::token::Token;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    operators: Vec<OperatorDefinition>
    errors: Vec<RoughError>,
    // Wouldn't be necessary if I could figure out how to return closures
    cur_token: Option<Token>,
}

static empty_early_error = RoughError::new("Source ended before making a valid expression");

impl Parser<'_> {
    pub fn new(mut lex: Lexer, operators: Vec<OperatorDefinition>) -> Parser {

        Parser {
            lexer: lex.filter(|token| !self.ignored(token)).peekable(),
            errors: Vec::new(),
            cur_token: lex.next(),
        }
    }

    pub fn current(&self) -> RoughResult<Expression> {
        match self.cur_token {
            Some(token) => Ok(token),
            None => Err(vec![empty_early_error]),
        }
    }

    pub fn peek(&self) -> RoughResult<Expression> {
        match self.lexer.peek() {
            Some(token) => Ok(token),
            None => Err(vec![empty_early_error]),
        }
    }

    pub fn next(&mut self) {
        self.cur_token = self.lexer.next();
    }

    fn next_if_equals(&mut self, expected: Token) -> bool {
        if expected == self.peek() {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn next_if_equals_result(&mut self, expected: Token) -> Result<(), String> {
        if !self.next_if_equals(expected) {
            Err(format!("Expected next token to be {} but it was {}", expected, self.current()?))
        } else {
            Ok(())
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors
    }

    // I suppose not ignoring whitespace might break a lot of code currently.
    // Should deal with this sooner rather than later.
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
        let token = self.current()?
        let prefix_parser = prefix_parse_lookup(&token)?;

        let mut exp = prefix_parser(self)?;

        // If it finds another Ident, it should assume a function call
        while precedence < self.token_precedence(&self.peek()) {
            let infix = match infix_parse_lookup(&self.peek()) {
                Some(infix_op) => infix_op,
                // Is this ok in this implementation?
                None => return Ok(left_exp),
            }

            self.next();
            exp = infix(self, exp)?;
        }

        Ok(exp)
    }

    fn token_precedence(&self, token: &Token) -> Precedence {
        if let Some(prec) = reserved_precedences(token.token_type) {
            return prec;
        }

        /// Check given operators
        if let TokenType::Ident(ident) = token.token_type {
            if let Some(op) = self.operators.iter().find(|op| op.identifier == ident) {
                return op.precedence
            }
        }

        Precedence::First
    }
}

fn parse_number(parser: &mut Parser) -> RoughResult<Expression> {
    match &parser.current()? {
        Token::Number(num) => Ok(Expression::Number(*num)),
        other => Err(vec![format!("Expected Number token, but got {}", other).to_string()]),
    }
}

fn parse_string_literal(parser: &mut Parser) -> RoughResult<Expression> {
    match &parser.current? {
        Token::Str(string) => Ok(Expression::Str(string.to_string())),
        other => Err(vec![format!("Expected Str token, but got {}", other).to_string()]),
    }
}

fn parse_function_parameters(parser: &mut Parser) -> RoughResult<Expression> {
    let mut params = vec![];

    parser.next();

    match parser.current()? {
        Token::Ident(name) => params.push(name.to_string()),
        other => return Err(vec![RoughError::new("Function parameter expected an Ident token but was {}", other)]),
    }

    while parser.peek()? == Token::Comma {
        parser.next();
        parser.next();

        match parser.current()? {
            Token::Ident(name) => params.push(name.to_string()),
            other => return Err(vec![RoughError::new("Function parameter expected an Ident token but was {}", other)]),
        }
    }

    parser.next_if_equals_result(Token::Pipe);

    Ok(params)
}

fn parse_function_literal(parser: &mut Parser) -> RoughResult<Expression> {
    Ok(Expression::Function(parse_function_parameters(parser), parser.parse_expression(Precedence::First)))
}

type PrefixParseFn = fn(parser: &mut Parser) -> RoughResult<Expression>;
type InfixParseFn = fn(parser: &mut Parser, left_exp: Expression) -> RoughResult<Expression>;

fn parse_if_expression(parser: &mut Parser) -> RoughResult<Expression> {
    let cond = parser.parse_expression(Precedence::First)?;

    if parser.peek() == Token::Else {
        let parsed_else = parser.parse_expression(Precedence::First)?;
        Ok(Expression::If(Box::new(cond, cons, parsed_else))
    } else {
        Ok(Expression::If(Box::new(cond, cons, None))
    }
}

fn parse_grouped_expression(parser: &mut Parser) -> RoughResult<Expression> {
    parser.next();
    let exp = parser.parse_expression(Precedence::First)?;

    match parser.current()? {
        Token::RParen => parser.next()
        other => return Err(vec![RoughError::new("Expected right paren at the end of expression, got {}", other)])
    };

    exp
}

// TODO actually implement map part.
fn parse_index_map_literal(parser: &mut Parser) -> RoughResult<Expression> {
    let mut elems: Vec<Expression> = vec![];

    if parser.peek() == Token::RBracket {
        parser.next();
        return Ok(elems)
    }

    parser.next();

    elems.push(parser.parse_expression(Precedence::First)?);

    while parser.peek() == Token::Comma {
        parser.next();
        parser.next();

        elems.push(parser.parse_expression(Precedence::Frist)?);
    }

    parser.next_if_equals_result(Token::RBracket)?;

    Ok(elems)
}

// I just realized that more needs to be done on the lexer level.
// Either have operator characters or some other way to lex operator as separate token
// Maybe have it configurable which symbols to use? Might be difficult to make consistant and is
// added overhead. Look and see how haskell and other languages do it.
// Might want to distinguish operators from identifier tokens then.
// Might need to figure out function calling here too.
fn parse_prefix_expression(parser: &mut Parser) -> RoughResult<Expression> {
    let op_token: Token = parser.current()?;

    let op_start = match op_token {
        Token::Ident(name) => name,
        other => return Err(vec![RoughError::new(format!("Should be an Ident token but got {}. Not sure how it even got here.", other))]),
    }

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
        .find(|op| op.name == op_start);

    parser.next();

    let right_exp = parser.parse_expression(op_def.precedence);
    
    Ok(Expression::Infix(Box::new(left_exp), op_def, Box::new(right_exp)))
}

fn prefix_parse_lookup(token: &Token) -> RoughResult<Expression> {
    let func = match token {
        Token::Number(_) => parse_number,
        Token::Str(_) => parse_string_literal,
        Token::Pipe => parse_function_literal,
        Token::Ident(_) => parse_prefix_expression,
        Token::If => parse_if_expression,
        Token::LParen => parse_grouped_expression,
        Token::LBracket => parse_index_map_literal,
        _ => RoughError::new(format!("prefix_parse_lookup doesn't have token {}", token).to_string()),
    };

    Ok(func)
}

fn infix_parse_lookup(token: &Token) -> Option<Expression> {
    match token {
        Token::Ident(_) => Some(parse_infix_expression),
        // TODO Do later. Make a built in function for now and add this in later.
        //Token::LBracket => Some(parse_index_expression),
        _ => None
    }
}

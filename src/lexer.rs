use crate::token::Token;

/// AKA Scanner
pub struct Lexer {
    pub source: Vec<char>,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            source: input.chars().collect(),
            start: 0,
            current: 0,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let cur_char = source[self.current - 1];
        self.current += 1;
        cur_char
    }

    pub fn peek(&mut self) -> Option<char> {
        self.input[self.current]
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.source.len() {
            return None
        }

        match self.advance() {
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '[' => Some(Token::LBracket),
            ',' => Some(Token::Comma),
            ']' => Some(Token::RBracket),
            '|' => Some(Token::Pipe),
            '\n' => Some(Token::Newline),
            //_ => 
        }
    }
}

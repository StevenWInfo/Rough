use crate::token::Token;
use crake::error::RoughResult;

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

    fn advance(&mut self) -> Option<char> {
        let cur_char = source[self.current - 1];
        self.current += 1;
        cur_char
    }

    fn peek(&mut self, extra: usize) -> Option<char> {
        self.input[self.current + usize]
    }

    fn read_identifier(&mut self) -> String {
        let 
    }
}

impl Iterator for Lexer {
    type Item = RoughResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.source.len() {
            return None
        }

        match self.advance() {
            '(' => Some(Ok(Token::LParen)),
            ')' => Some(Ok(Token::RParen)),
            '[' => Some(Ok(Token::LBracket)),
            ',' => Some(Ok(Token::Comma)),
            ']' => Some(Ok(Token::RBracket)),
            '|' => Some(Ok(Token::Pipe)),
            '#' => Some(Ok(Token::Hash)),
            // TODO escaping double quotes
            '\n' => {
                if self.peek(0) == '\r' {
                    self.advance();
                }
                Some(Ok(Token::Newline))
            },
            '\r' => {
                if self.peek(0) == '\n' {
                    self.advance();
                }
                Some(Ok(Token::Newline))
            }
            ':' => if self.peek(0) == '=' {
                Some(Ok(Token::Assign))
            } else {
                Some(Ok(Token::Colon))
            },
            ' ' => Some(),
            '"' => Some(Ok(Token::Str(self.read_string()))),
            other => if other.is_ascii_digit() {
                Some(Ok(Token::Int(self.read_number())))
            } else if other.is_ascii_alphabetic() || ch == '_' {
                lookup_ident(self.read_identifier())
            } else {
                Some(Err(vec!(RoughError::new("Lexer error with character {}"))))
            }
                /* I suppose it's better to limit so things like null characters and other devious
                 * characters don't get interpreted as identifiers.
            } else {
                lookup_ident(self.read_identifier())
            }
            */
        }
    }
}

fn check_keyword(word: String) -> Option<Token> {
    match word.as_str() {
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        ":=" => Some(Token::Assign),
        _ => None
    }
}

fn lookup_ident(ident: String) -> Token {
    match check_keyword(ident) {
        Some(token) => token,
        None => Token::Ident(ident)
    }
}

use crate::token::{ TokenType, Token };
use crate::error::{ RoughError, RoughResult };
use std::str::CharIndices;
use std::iter::Peekable;

/// AKA Scanner
pub struct Lexer<'a> {
    source: &'a str,
    source_iter: Peekable<CharIndices<'a>>,
    pub errors: Vec<RoughError>,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            source: input,
            source_iter: input.char_indices().peekable(),
            errors: Vec::new(),
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut string = format!("{}", first);

        while let Some((_, ch)) = self.source_iter.peek() {
            if !is_letter(*ch) {
                return string
            };
            string = format!("{}{}", string, ch.clone());
            self.source_iter.next();
        }

        string
    }

    // TODO implement character escaping
    fn read_string(&mut self) -> RoughResult<String> {
        let mut string = "".to_string();

        let mut closed = false;

        while let Some((_, ch)) = self.source_iter.next() {
            if ch == '"' {
                closed = true;
                break
            };

            string = format!("{}{}", string, ch);
        }

        if !closed {
            Err(vec!(RoughError::new("File ended before string closed".to_string())))
        } else {
            Ok(string.to_string())
        }
    }

    // Could put operator definition here or in parser.
    fn read_operator(&mut self, first: char) -> RoughResult<String> {
        let mut op = vec![first];

        while let Some((_, ch)) = self.source_iter.peek() {
            if !is_op_char(ch) {
                //return Ok(op.collect());
                break;
            };

            op.push(ch);
            self.source_iter.next();
        }

        Ok(op.iter().collect())
    }

    fn read_number(&mut self, first: char) -> f64 {
        let mut number = format!("{}", first);

        while let Some((_, ch)) = self.source_iter.peek() {
            if !ch.is_ascii_digit() {
                //return number.parse::<f64>().unwrap()
                break;
            };
            number = format!("{}{}", number, ch.clone());
            self.source_iter.next();
        }

        number.parse::<f64>().unwrap()
    }

    fn read_comment(&mut self) -> RoughResult<String> {
        let mut comment = "".to_string();

        while let Some((_, ch)) = self.source_iter.peek() {
            self.source_iter.next();
            if ch == '\n' {
                break
            } else if ch == '*' && self.source_iter.peek().map(|peek_ch| peek_ch.1 == '#').unwrap_or(false) {
                self.source_iter.next();
                break
            };

            comment = format!("{}{}", comment, ch);
        }

        Ok(comment.to_string())
    }

    fn handle_error(&mut self, errors: Vec<RoughError>) -> Option<Token> {
        self.errors.append(error.clone());
        self.next()
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, cur_char) = match self.source_iter.next() {
            Some(char_index) => char_index,
            None => return None
        };

        let token_type = match cur_char {
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '[' => TokenType::LBracket,
            ',' => TokenType::Comma,
            ']' => TokenType::RBracket,
            '|' => TokenType::Pipe,
            '#' => match self.read_comment() {
                Ok(comment) => TokenType::Comment(comment),
                Err(error) => return self.handle_error(error),
            }
            '\n' => {
                if let Some((_, '\r')) = self.source_iter.peek() {
                    self.source_iter.next();
                }
                TokenType::Newline
            },
            '\r' => {
                if let Some((_, '\n')) = self.source_iter.peek() {
                    self.source_iter.next();
                }
                TokenType::Newline
            }
            ':' => if let Some((_, '=')) = self.source_iter.peek() {
                self.source_iter.next();
                TokenType::Assign
            } else {
                TokenType::Colon
            },
            '\t' => TokenType::Tab,
            // Maybe make this configurable
            ' ' => if self.source.len() >= start + 4 && &self.source[start..(start + 4)] == "    " {
                // I feel like there's got to be some way to do something like this.
                //self.source_iter.take(3).for_each(drop);
                self.source_iter.next();
                self.source_iter.next();
                self.source_iter.next();
                TokenType::Tab
            } else {
                TokenType::Space
            },
            // TODO escaping double quotes
            '"' => match self.read_string() {
                Ok(string) => TokenType::Str(string),
                Err(error) => return self.handle_error(error),
            },

            other if is_op_char(other) => match self.read_operator(other) {
                Ok(op) => TokenType::Operator(op),
                Err(error) => return self.handle_error(error),
            },

            other if other.is_ascii_digit() => TokenType::Number(self.read_number(other)),
                
            other if is_letter(other) => lookup_ident(self.read_identifier(other)),

            other => return self.handle_error(
                RoughError::new(
                    vec![
                    format!(
                        "Lexer error with character {}",
                        other
                        )
                    ]
                    )
                )
        };

        Some(Token::new(token_type, start))
    }
}

fn check_keyword(word: &str) -> Option<TokenType> {
    match word {
        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),
        "in" => Some(TokenType::In),
        _ => None
    }
}

fn lookup_ident(ident: String) -> TokenType {
    match check_keyword(&ident) {
        Some(token) => token,
        None => TokenType::Ident(ident)
    }
}

/// Want to expand this later, but wary of things like null characters.
fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_op_char(ch: char) -> bool {
    OPERATOR_CHARACTERS.iter()
        .position(|op_ch| op_ch == ch)
        .is_some()
}
/// Want to expand this too, but need to start somewhere.
const OPERATOR_CHARACTERS: [char; 19] = ['!', '$', '%', '&', '*', '+', '.', '/', '<', '=', '>', '?', '@', '\\', '^', '-', '~', '{', '}'];

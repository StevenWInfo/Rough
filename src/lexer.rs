use crate::token::{ TokenType, Token };
use crate::error::{ RoughError, RoughResult };
use std::str::CharIndices;
use std::iter::Peekable;

/// AKA Scanner
pub struct Lexer<'a> {
    // Could maybe be a string slice with lifetimes?
    source: &'a str,
    // Should probably use a more limited lifetime.
    source_iter: Peekable<CharIndices<'a>>,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            source: input,
            source_iter: input.char_indices().peekable(),
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

    fn read_string(&mut self) -> RoughResult<String> {
        let mut string = "".to_string();

        while let Some((_, ch)) = self.source_iter.next() {
            if ch == '"' {
                break
            };

            string = format!("{}{}", string, ch);
            
            if self.source_iter.peek() == None {
                return Err(vec!(RoughError::new("File ended before string closed".to_string())));
            };
        }

        Ok(string.to_string())
    }

    fn read_number(&mut self, first: char) -> f64 {
        let mut number = format!("{}", first);

        while let Some((_, ch)) = self.source_iter.peek() {
            if !ch.is_ascii_digit() {
                return number.parse::<f64>().unwrap()
            };
            number = format!("{}{}", number, ch.clone());
            self.source_iter.next();
        }

        number.parse::<f64>().unwrap()
    }
}

impl Iterator for Lexer<'_> {
    type Item = RoughResult<Token>;

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
            '#' => TokenType::Hash,
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
                Err(msg) => return Some(Err(msg)),
            }
            other => if other.is_ascii_digit() {
                TokenType::Number(self.read_number(other))
            } else if is_letter(other) {
                lookup_ident(self.read_identifier(other))
            } else {
                return Some(Err(vec![RoughError::new(
                               format!("Lexer error with character {}", other)
                               )]));
            }
        };

        Some(Ok(Token::new(token_type, start)))
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

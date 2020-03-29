use crate::token::{ TokenType, Token };
use crake::error::RoughResult;

/// AKA Scanner
pub struct Lexer {
    source: &str,
    source_iter: Peekable<CharIndices>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let input_str = input.as_str();
        Lexer {
            source: input_str,
            source_iter: input_str.char_indices().peekable(),
        }
    }

    fn read_identifier(&mut self, first: char) -> String {
        let string = format!("{}", first);

        while Some(ch) = self.source_iter.peek() {
            if !is_letter(ch) {
                return string
            };
            self.source_iter.next();
            string = format!("{}{}", string, ch)
        }

        string
    }

    fn read_string(&mut self, first: char) -> RoughResult<String> {
        let string = format!("{}", first);
        for ch in self.source_iter {
            if ch == '"' {
                break
            };

            string = format!("{}{}", string, ch);
            
            if self.source_iter.peek() == None {
                return Err(vec!(RoughError::new("File ended before string closed"
            };
        }

        Ok(string)
    }

    fn read_number(&mut self, first: char) -> f64 {
        let number = format!("{}", first);

        while Some(ch) = self.source_iter.peek() {
            if !is_letter(ch) {
                return number.parse::<i64>().unwrap()
            };
            self.source_iter.next();
            number = format!("{}{}", number, ch)
        }

        number.parse::<i64>().unwrap()
    }
}

impl Iterator for Lexer {
    type Item = RoughResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.source.len() {
            return None
        }

        let (cur_char, start) = match self.source_iter.next() {
            Some(char_index) => char_index,
            None => Return None
        }

        let token_type = match cur_char {
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '[' => TokenType::LBracket,
            ',' => TokenType::Comma,
            ']' => TokenType::RBracket,
            '|' => TokenType::Pipe,
            '#' => TokenType::Hash,
            '\n' => {
                if let Some('\r') = self.source_iter.peek() {
                    self.source_iter.next();
                }
                TokenType::Newline
            },
            '\r' => {
                if let Some('\n') = self.source_iter.peek() {
                    self.source_iter.next();
                }
                TokenType::Newline
            }
            ':' => if let Some('=') = self.source_iter.peek() {
                TokenType::Assign
            } else {
                TokenType::Colon
            },
            '\t' => TokenType::Tab,
            // Maybe make this configurable
            ' ' => if source.len() >= start + 4 && self.source[start..(start + 3)] == "    " {
                self.source_iter.take(3).collect();
                TokenType::Tab
            } else {
                TokenType::Space
            },
            // TODO escaping double quotes
            '"' => TokenType::Str(self.read_string(other)?),
            other => if other.is_ascii_digit() {
                TokenType::Int(self.read_number(other))
            } else if is_letter(other) {
                lookup_ident(self.read_identifier(other))
            } else {
                return Err(vec![RoughError::new(
                               format!("Lexer error with character {}", other)
                               )]);
            }
        };

        Some(Ok(Token::new(token_type, start)))
    }
}

fn check_keyword(word: String) -> Option<Token> {
    match word.as_str() {
        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),
        "in" => Some(TokenType::In),
        _ => None
    }
}

fn lookup_ident(ident: String) -> Token {
    match check_keyword(ident) {
        Some(token) => token,
        None => TokenType::Ident(ident)
    }
}

/// Want to expand this later, but wary of things like null characters.
fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

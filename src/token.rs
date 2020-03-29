use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    //Illegal, // With Result is this necessary?
    // Trying it without this token since Lexer is an iterator.
    //EOF,

    Ident(String),
    Number(f64),
    Str(String),
    LParen,
    RParen,
    LBracket,
    Colon,
    Comma,
    RBracket,
    Hash,
    If,
    Else,
    Assign,
    In,
    Pipe,
    Space,
    Tab,
    Newline,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match &*self {
            //Token::Illegal => write!(f, "Illegal"),
            //Token::EOF => write!(f, "EOF"),
            Token::Ident(name) => write!(f, "{}", name),
            Token::Number(num) => write!(f, "{}", num),
            Token::Str(string) => write!(f, "{}", string),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Hash => write!(f, "#"),
            Token::RBracket => write!(f, "]"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Assign => write!(f, ":="),
            Token::In => write!(f, "in"),
            Token::Pipe => write!(f, "|"),
            Token::Space => write!(f, " "),
            // Might want to make this configurable
            Token::Tab => write!(f, "    "),
            Token::Newline => write!(f, "\n"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    /// For finding the token later, potentially when showing errors.
    /// It's the position of the first character scanned in token.
    position: usize,
}

impl Token {
    pub fn new (token_type: TokenType, position: usize) -> Token{
        Token {
            token_type: token_type,
            position: position,
        }
    }
}

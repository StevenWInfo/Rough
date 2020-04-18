use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    //Illegal, // With Result is this necessary?
    // Trying it without this token since Lexer is an iterator.
    //EOF,

    Ident(String),
    Number(f64),
    Str(String),
    Comment(String),
    Operator(String),
    LParen,
    RParen,
    LBracket,
    Colon,
    Comma,
    RBracket,
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
            //TokenType::Illegal => write!(f, "Illegal"),
            //TokenType::EOF => write!(f, "EOF"),
            TokenType::Ident(name) => write!(f, "{}", name),
            TokenType::Number(num) => write!(f, "{}", num),
            TokenType::Str(string) => write!(f, "{}", string),
            TokenType::Comment(comment) => write!(f, "{}", comment),
            TokenType::Operator(name) => write!(f, "{}", name),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBracket => write!(f, "["),
            TokenType::Colon => write!(f, ":"),
            TokenType::Comma => write!(f, ","),
            TokenType::Hash => write!(f, "#"),
            TokenType::RBracket => write!(f, "]"),
            TokenType::If => write!(f, "if"),
            TokenType::Else => write!(f, "else"),
            TokenType::Assign => write!(f, ":="),
            TokenType::In => write!(f, "in"),
            TokenType::Pipe => write!(f, "|"),
            TokenType::Space => write!(f, " "),
            // Might want to make this configurable
            TokenType::Tab => write!(f, "    "),
            TokenType::Newline => write!(f, "\n"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

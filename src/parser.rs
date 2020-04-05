

pub enum Precedence {
    First = 0,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    errors: Vec<RoughError>,
    // Might not be necessary? Will need to reevaluate
    cur_token: Option<Token>,
}

impl Parser<'_> {
    pub fn new(mut lex: Lexer) -> Parser {
        let cur_token = match lex.next() {
        }
        Parser {
            lexer: lex.peekable(),
            errors: Vec::new(),
            cur_token: lex.next(),
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors
    }

    pub fn parse_program(&mut self) -> Program {
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
    }
}

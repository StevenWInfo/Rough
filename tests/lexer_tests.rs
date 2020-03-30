use rough::token::{ Token, TokenType };
use rough::lexer::Lexer;
use rough::error::RoughResult;

#[test]
fn test_simple_tokens() {
    let tests = vec![
        ("()[],|#".to_string(), vec![
         Token::new(TokenType::LParen, 0),
         Token::new(TokenType::RParen, 1),
         Token::new(TokenType::LBracket, 2),
         Token::new(TokenType::RBracket, 3),
         Token::new(TokenType::Comma, 4),
         Token::new(TokenType::Pipe, 5),
         Token::new(TokenType::Hash, 6),
        ]),
        ("foo := |x| 53 in print
            \"bar\"".to_string(), vec![
            Token::new(TokenType::Ident("foo".to_string()), 0),
            Token::new(TokenType::Space, 3),
            Token::new(TokenType::Assign, 4),
            Token::new(TokenType::Space, 6),
            Token::new(TokenType::Pipe, 7),
            Token::new(TokenType::Ident("x".to_string()), 8),
            Token::new(TokenType::Pipe, 9),
            Token::new(TokenType::Space, 10),
            Token::new(TokenType::Number(53.0), 11),
            Token::new(TokenType::Space, 13),
            Token::new(TokenType::In, 14),
            Token::new(TokenType::Space, 16),
            Token::new(TokenType::Ident("print".to_string()), 17),
            Token::new(TokenType::Newline, 22),
            Token::new(TokenType::Tab, 23),
            Token::new(TokenType::Tab, 27),
            Token::new(TokenType::Tab, 31),
            Token::new(TokenType::Str("bar".to_string()), 35),
            ]),
    ];

    for (test, (given, expected)) in tests.iter().enumerate() {
        let lexer = Lexer::new(given);
        let lexer_output: Vec<RoughResult<Token>> = lexer.collect();
        let expected_result: Vec<RoughResult<Token>> = expected.into_iter()
            .map(|exp| Ok(exp.clone()))
            .collect();
        //assert_eq!(lexer_output, expected_result, "Test{}: {:?} not equal to {:?}", test, lexer_output, expected_result);
        for (i, expect) in expected_result.iter().enumerate() {
            assert_eq!(lexer_output[i], *expect, "Test{}, token{}: {:?} not equal to {:?}", test, i, lexer_output[i], expect);
        }
    }
}

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
    ];

    for (test, (given, expected)) in tests.iter().enumerate() {
        let lexer = Lexer::new(given);
        let lexer_output: Vec<RoughResult<Token>> = lexer.collect();
        let expected_result: Vec<RoughResult<Token>> = expected.into_iter().map(|exp| Ok(exp.clone())).collect();
        assert_eq!(lexer_output, expected_result, "Test{:?}: {:?} not equal to {:?}", test, lexer_output, expected_result);
    }
}

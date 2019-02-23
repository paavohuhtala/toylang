use toylang::parser::{ParseError, ParseErrorCtx};
use toylang::tokens::TokenKind;
use toylang::{eval, EvalError};

#[test]
pub fn missing_semicolon() {
  let result = eval("let x = 9");
  assert_eq!(
    Err(EvalError::ParseError(ParseErrorCtx(
      9,
      ParseError::UnexpectedToken {
        expected: vec![TokenKind::Semicolon],
        was: TokenKind::EOF
      }
    ))),
    result
  );
}

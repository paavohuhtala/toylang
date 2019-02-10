use crate::ast::{Expression, Statement};
use crate::token_stream::{LexerError, LexerErrorCtx, TokenStream};
use crate::tokens::{Token, TokenKind};
use crate::utils::ResultExt;

#[derive(Debug)]
pub enum ParseError {
  UnexpectedEof,
  LexerError(LexerError),
  UnexpectedToken { expected: TokenKind, was: TokenKind },
}

#[derive(Debug)]
pub struct ParseErrorCtx(usize, ParseError);

impl From<LexerErrorCtx> for ParseErrorCtx {
  fn from(x: LexerErrorCtx) -> ParseErrorCtx {
    ParseErrorCtx(x.0, ParseError::LexerError(x.1))
  }
}

pub type ParseResult<T> = Result<T, ParseErrorCtx>;

pub struct Parser<'a> {
  lexer: &'a mut TokenStream<'a>,
}

impl<'a> TokenStream<'a> {
  pub fn peek(&mut self) -> ParseResult<&Token> {
    let offset = self.byte_offset();
    self.peek_token().err_into()
  }

  pub fn take(&mut self) -> ParseResult<Token> {
    let offset = self.byte_offset();
    self.take_token().err_into()
  }

  pub fn take_of(&mut self, kind: TokenKind) -> ParseResult<Token> {
    let offset = self.byte_offset();
    self.take().and_then(|token| {
      let token_kind = token.to_kind();
      if token_kind == kind {
        Ok(token)
      } else {
        Err(ParseErrorCtx(
          offset,
          ParseError::UnexpectedToken {
            expected: kind,
            was: token_kind,
          },
        ))
      }
    })
  }

  pub fn take_identifier(&mut self) -> ParseResult<&str> {
    self
      .take_of(TokenKind::Identifier)
      .map(|token| match token {
        Token::Identifier(name) => name,
        _ => unsafe {
          std::hint::unreachable_unchecked();
        },
      })
  }

  pub fn take_integer(&mut self) -> ParseResult<i64> {
    self.take_of(TokenKind::Integer).map(|token| match token {
      Token::Integer(value) => value,
      _ => unsafe {
        std::hint::unreachable_unchecked();
      },
    })
  }
}

impl<'a> Parser<'a> {
  fn parse_declaration(&mut self) -> ParseResult<Statement> {
    self.lexer.take_of(TokenKind::Const)?;
    let name = self.lexer.take_identifier()?.to_string();
    self.lexer.take_of(TokenKind::Equals)?;
    let value = self.lexer.take_integer()?;
    self.lexer.take_of(TokenKind::Semicolon)?;

    Ok(Statement::DeclareVariable(
      name,
      Expression::IntegerLiteral(value),
    ))
  }

  pub fn parse_statement(&mut self) -> ParseResult<Statement> {
    let first = self.lexer.peek()?;

    match first {
      Token::Const | Token::Let => self.parse_declaration(),
      _ => unimplemented!("Unimplemented statement."),
    }
  }
}

#[cfg(test)]
mod parser_tests {
  use super::{Parser, TokenStream};

  #[test]
  fn parse_declaration() {
    let mut parser = Parser {
      lexer: &mut TokenStream::new("const x = 10;"),
    };

    use crate::ast::Expression::*;
    use crate::ast::Statement::*;

    let statement = parser.parse_declaration();

    match statement {
      Ok(DeclareVariable(ref name, IntegerLiteral(10))) if name == "x" => {}
      _ => panic!("Unexpected AST: {:#?}", statement),
    };
  }
}

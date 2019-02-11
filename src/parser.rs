use crate::ast::{Expression, Statement};
use crate::token_stream::{LexerError, LexerErrorCtx, TokenStream};
use crate::tokens::{Token, TokenKind};

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
  pub fn take_of(&mut self, kind: TokenKind) -> ParseResult<Token> {
    let offset = self.byte_offset();
    let token = self.take()?;
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
    self.lexer.take_of(TokenKind::Let)?;

    let is_mutable = if let Token::Mut = self.lexer.peek()? {
      self.lexer.take()?;
      true
    } else {
      false
    };

    let name = self.lexer.take_identifier()?.to_string();
    self.lexer.take_of(TokenKind::Equals)?;
    let value = self.lexer.take_integer()?;
    self.lexer.take_of(TokenKind::Semicolon)?;

    Ok(Statement::DeclareVariable {
      name,
      initial_value: Expression::IntegerLiteral(value),
      is_mutable,
    })
  }

  pub fn parse_statement(&mut self) -> ParseResult<Statement> {
    let first = self.lexer.peek()?;

    match first {
      Token::Let => self.parse_declaration(),
      _ => unimplemented!("Unimplemented statement."),
    }
  }
}

#[cfg(test)]
mod parser_tests {
  use super::{Parser, TokenStream};
  use crate::ast::Expression::*;
  use crate::ast::Statement::*;

  #[test]
  fn parse_declaration() {
    let mut parser = Parser {
      lexer: &mut TokenStream::new("let x = 10;"),
    };

    let statement = parser.parse_declaration();

    match statement {
      Ok(DeclareVariable {
        ref name,
        is_mutable: false,
        initial_value: IntegerLiteral(10),
      }) if name == "x" => {}
      _ => panic!("Unexpected AST: {:#?}", statement),
    };
  }

  #[test]
  fn parse_mut_declaration() {
    let mut parser = Parser {
      lexer: &mut TokenStream::new("let mut coolAndMutable = 0;"),
    };

    let statement = parser.parse_declaration();

    match statement {
      Ok(DeclareVariable {
        ref name,
        is_mutable: true,
        initial_value: IntegerLiteral(0),
      }) if name == "coolAndMutable" => {}
      _ => panic!("Unexpected AST: {:#?}", statement),
    };
  }
}

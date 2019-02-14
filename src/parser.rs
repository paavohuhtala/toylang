use crate::ast::{Expression, ExpressionCtx, Statement, StatementCtx};
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
  pub fn take_of(&mut self, kind: TokenKind) -> ParseResult<(usize, Token)> {
    let token = self.take_pos()?;
    let token_kind = token.1.to_kind();
    if token_kind == kind {
      Ok(token)
    } else {
      Err(ParseErrorCtx(
        token.0,
        ParseError::UnexpectedToken {
          expected: kind,
          was: token_kind,
        },
      ))
    }
  }

  pub fn take_identifier(&mut self) -> ParseResult<(usize, &str)> {
    self
      .take_of(TokenKind::Identifier)
      .map(|token| match token.1 {
        Token::Identifier(name) => (token.0, name),
        _ => unsafe {
          std::hint::unreachable_unchecked();
        },
      })
  }

  pub fn take_integer(&mut self) -> ParseResult<(usize, i64)> {
    self.take_of(TokenKind::Integer).map(|token| match token.1 {
      Token::Integer(value) => (token.0, value),
      _ => unsafe {
        std::hint::unreachable_unchecked();
      },
    })
  }
}

impl<'a> Parser<'a> {
  fn parse_expression(&mut self) -> ParseResult<ExpressionCtx> {
    let (pos, first) = self.lexer.take_pos()?;

    match first {
      Token::Integer(i) => Ok(ExpressionCtx(pos, Expression::IntegerLiteral(i))),
      _ => unimplemented!(),
    }
  }

  fn parse_declaration(&mut self) -> ParseResult<StatementCtx> {
    let (pos, _) = self.lexer.take_of(TokenKind::Let)?;

    let is_mutable = if let Token::Mut = self.lexer.peek()? {
      self.lexer.take()?;
      true
    } else {
      false
    };

    let name = self.lexer.take_identifier()?.1.to_string();
    self.lexer.take_of(TokenKind::Equals)?;
    let initial_value = self.parse_expression()?;
    self.lexer.take_of(TokenKind::Semicolon)?;

    Ok(StatementCtx(
      pos,
      Statement::DeclareVariable {
        name,
        initial_value,
        is_mutable,
      },
    ))
  }

  pub fn parse_statement(&mut self) -> ParseResult<StatementCtx> {
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
  use crate::ast::{ExpressionCtx, StatementCtx};

  #[test]
  fn parse_declaration() {
    let mut parser = Parser {
      lexer: &mut TokenStream::new("let x = 10;"),
    };

    let statement = parser.parse_statement();

    match statement {
      Ok(StatementCtx(
        0,
        DeclareVariable {
          ref name,
          is_mutable: false,
          initial_value: ExpressionCtx(8, IntegerLiteral(10)),
        },
      )) if name == "x" => {}
      _ => panic!("Unexpected AST: {:#?}", statement),
    };
  }

  #[test]
  fn parse_mut_declaration() {
    let mut parser = Parser {
      lexer: &mut TokenStream::new("let mut coolAndMutable = 0;"),
    };

    let statement = parser.parse_statement();

    match statement {
      Ok(StatementCtx(
        0,
        DeclareVariable {
          ref name,
          is_mutable: true,
          initial_value: ExpressionCtx(25, IntegerLiteral(0)),
        },
      )) if name == "coolAndMutable" => {}
      _ => panic!("Unexpected AST: {:#?}", statement),
    };
  }
}

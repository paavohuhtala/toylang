use crate::ast::Program;
use crate::ast::{Expression, ExpressionCtx, IdentifierCtx, Statement, StatementCtx};
use crate::token_stream::{LexerError, LexerErrorCtx, TokenStream};
use crate::tokens::{Token, TokenKind};

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
  UnexpectedEof,
  LexerError(LexerError),
  UnexpectedToken { expected: TokenKind, was: TokenKind },
}

#[derive(Debug, Eq, PartialEq)]
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

  pub fn take_identifier(&mut self) -> ParseResult<(usize, IdentifierCtx)> {
    self
      .take_of(TokenKind::Identifier)
      .map(|token| match token.1 {
        Token::Identifier(name) => (token.0, IdentifierCtx(token.0, name.to_string())),
        _ => unsafe {
          std::hint::unreachable_unchecked();
        },
      })
  }

  pub fn take_integer(&mut self) -> ParseResult<(usize, i128)> {
    self.take_of(TokenKind::Integer).map(|token| match token.1 {
      Token::Integer(value) => (token.0, value),
      _ => unsafe {
        std::hint::unreachable_unchecked();
      },
    })
  }
}

impl<'a> Parser<'a> {
  pub fn new(lexer: &'a mut TokenStream<'a>) -> Parser<'a> {
    Parser { lexer }
  }

  fn parse_expression(&mut self) -> ParseResult<ExpressionCtx> {
    let (pos, first) = self.lexer.take_pos()?;

    match first {
      Token::Integer(i) => Ok(ExpressionCtx(pos, Expression::IntegerConstant(i))),
      Token::Identifier(x) => Ok(ExpressionCtx(pos, Expression::Local(x.to_string()))),
      _ => unimplemented!(),
    }
  }

  fn parse_assignment(&mut self) -> ParseResult<StatementCtx> {
    let (pos, local) = self.lexer.take_identifier()?;
    self.lexer.take_of(TokenKind::Equals)?;
    let value = self.parse_expression()?;
    self.lexer.take_of(TokenKind::Semicolon)?;

    Ok(StatementCtx(pos, Statement::AssignLocal { local, value }))
  }

  fn parse_declaration(&mut self) -> ParseResult<StatementCtx> {
    let (pos, _) = self.lexer.take_of(TokenKind::Let)?;

    let is_mutable = if let Token::Mut = self.lexer.peek()? {
      self.lexer.take()?;
      true
    } else {
      false
    };

    let name = self.lexer.take_identifier()?.1;

    let initial_type = match self.lexer.peek()? {
      Token::Colon => {
        self.lexer.take()?;
        Some(self.lexer.take_identifier()?.1)
      }
      _ => None,
    };

    self.lexer.take_of(TokenKind::Equals)?;
    let initial_value = self.parse_expression()?;
    self.lexer.take_of(TokenKind::Semicolon)?;

    Ok(StatementCtx(
      pos,
      Statement::DeclareVariable {
        name,
        initial_type,
        initial_value,
        is_mutable,
      },
    ))
  }

  pub fn parse_block(&mut self) -> ParseResult<StatementCtx> {
    let (pos, _) = self.lexer.take_of(TokenKind::LBrace)?;

    let mut inner = Vec::new();

    loop {
      let next = self.lexer.peek()?;
      if *next == Token::RBrace {
        self.lexer.take()?;
        break;
      }

      inner.push(self.parse_statement()?);
    }

    Ok(StatementCtx(pos, Statement::Block { inner }))
  }

  pub fn parse_statement(&mut self) -> ParseResult<StatementCtx> {
    let first = self.lexer.peek()?;

    match first {
      Token::Let => self.parse_declaration(),
      Token::LBrace => self.parse_block(),
      Token::Identifier(_) => self.parse_assignment(),
      _ => unimplemented!("Unimplemented statement."),
    }
  }

  pub fn parse_program(&mut self) -> ParseResult<Program> {
    let mut statements = Vec::new();

    loop {
      let next = self.lexer.peek()?;
      if *next == Token::EOF {
        break;
      }
      statements.push(self.parse_statement()?);
    }

    Ok(Program(statements))
  }
}

#[cfg(test)]
mod parser_tests {
  use super::{Parser, TokenStream};
  use crate::ast::Expression::*;
  use crate::ast::Statement::*;
  use crate::ast::{ExpressionCtx, IdentifierCtx, StatementCtx};

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
          initial_type: None,
          initial_value: ExpressionCtx(8, IntegerConstant(10)),
        },
      )) if name.1 == "x" => {}
      _ => panic!("Unexpected AST: {:#?}", statement),
    };
  }

  #[test]
  fn parse_declaration_with_type_annotation() {
    let mut parser = Parser {
      lexer: &mut TokenStream::new("let x : i32 = 10;"),
    };

    let statement = parser.parse_statement();

    match statement {
      Ok(StatementCtx(
        0,
        DeclareVariable {
          name: IdentifierCtx(4, ref name),
          is_mutable: false,
          initial_type: Some(IdentifierCtx(8, ref type_name)),
          initial_value: ExpressionCtx(14, IntegerConstant(10)),
        },
      )) if name == "x" && type_name == "i32" => {}
      _ => panic!("Unexpected AST: {:#?}", statement),
    };
  }

  #[test]
  fn parse_mut_declaration() {
    let mut parser = Parser {
      lexer: &mut TokenStream::new("let mut mutable_x = 0;"),
    };

    let statement = parser.parse_statement();

    match statement {
      Ok(StatementCtx(
        0,
        DeclareVariable {
          name: IdentifierCtx(8, ref name),
          is_mutable: true,
          initial_type: None,
          initial_value: ExpressionCtx(20, IntegerConstant(0)),
        },
      )) if name == "mutable_x" => {}
      _ => panic!("Unexpected AST: {:#?}", statement),
    };
  }

  #[test]
  fn parse_block() {
    let mut parser = Parser {
      lexer: &mut TokenStream::new("{ let x = 0; }"),
    };

    let statement = parser.parse_statement();

    assert_eq!(
      Ok(StatementCtx(
        0,
        Block {
          inner: vec![StatementCtx(
            2,
            DeclareVariable {
              name: IdentifierCtx(6, "x".to_string()),
              initial_type: None,
              is_mutable: false,
              initial_value: ExpressionCtx(10, IntegerConstant(0))
            }
          )]
        }
      )),
      statement
    );
  }
}

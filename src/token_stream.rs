use crate::char_stream::CharStream;
use crate::parse_utils;
use crate::tokens::{OperatorToken, Token};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LexerError {
  UnknownToken(String),
  InvalidNumber(String),
  UnterminatedString,
  UnexpectedEof,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LexerErrorCtx(pub usize, pub LexerError);

pub type LexerResult<T> = Result<T, LexerErrorCtx>;

pub struct TokenStream<'a> {
  stream: CharStream<'a>,
  lookahead: Option<(usize, Token<'a>)>,
}

impl<'a> TokenStream<'a> {
  pub fn new(src: &'a str) -> TokenStream<'a> {
    TokenStream {
      stream: CharStream::from_str(src),
      lookahead: None,
    }
  }

  fn read_keyword_or_identifier(&mut self) -> LexerResult<Token<'a>> {
    let keyword_or_identifier = self.stream.take_while_indexed(|(i, x)| {
      if i == 0 {
        parse_utils::is_valid_identifier_first(x)
      } else {
        parse_utils::is_valid_in_identifier(x)
      }
    });

    match keyword_or_identifier {
      "let" => Ok(Token::Let),
      "mut" => Ok(Token::Mut),
      otherwise => Ok(Token::Identifier(otherwise)),
    }
  }

  fn read_number(&mut self) -> LexerResult<Token<'a>> {
    let offset = self.byte_offset();
    let chars = self.stream.take_while(|c| c.is_digit(10));
    let parsed = chars
      .parse()
      .map_err(|_| LexerErrorCtx(offset, LexerError::InvalidNumber(chars.to_string())))?;
    Ok(Token::Integer(parsed))
  }

  fn read_token(&mut self) -> LexerResult<(usize, Token<'a>)> {
    use Token::*;

    self.stream.skip_whitespace();

    let offset = self.byte_offset();

    if self.stream.remaining() == 0 {
      return Ok((offset, Token::EOF));
    }

    let fst = self
      .stream
      .peek()
      .ok_or_else(|| LexerErrorCtx(offset, LexerError::UnexpectedEof))?;

    (match fst {
      '(' => {
        self.stream.advance();
        Ok(LParen)
      }
      ')' => {
        self.stream.advance();
        Ok(RParen)
      }
      '{' => {
        self.stream.advance();
        Ok(LBrace)
      }
      '}' => {
        self.stream.advance();
        Ok(RBrace)
      }
      '=' => {
        self.stream.advance();
        Ok(Equals)
      }
      ';' => {
        self.stream.advance();
        Ok(Semicolon)
      }
      ':' => {
        self.stream.advance();
        Ok(Colon)
      }
      '+' => {
        self.stream.advance();
        Ok(Operator(OperatorToken::Plus))
      }
      '-' => {
        self.stream.advance();
        Ok(Operator(OperatorToken::Minus))
      }
      '*' => {
        self.stream.advance();
        Ok(Operator(OperatorToken::Asterisk))
      }
      '0'..='9' => self.read_number(),
      'A'..='z' => self.read_keyword_or_identifier(),
      _ => Err(LexerErrorCtx(
        offset,
        LexerError::UnknownToken(fst.to_string()),
      )),
    })
    .map(|x| (offset, x))
  }

  pub fn peek_pos(&mut self) -> LexerResult<&(usize, Token)> {
    if self.lookahead.is_none() {
      self.lookahead = Some(self.read_token()?);
    }

    Ok(&self.lookahead.as_ref().unwrap_or_else(|| unsafe {
      std::hint::unreachable_unchecked();
    }))
  }

  pub fn peek(&mut self) -> LexerResult<&Token> {
    self.peek_pos().map(|x| &x.1)
  }

  pub fn take_pos(&mut self) -> LexerResult<(usize, Token)> {
    if let Some(token) = self.lookahead {
      self.lookahead = None;
      Ok(token)
    } else {
      self.read_token()
    }
  }

  pub fn take(&mut self) -> LexerResult<Token> {
    self.take_pos().map(|x| x.1)
  }

  pub fn byte_offset(&self) -> usize {
    self.stream.byte_offset()
  }
}

#[cfg(test)]
mod token_stream_tests {
  use super::{Token, TokenStream};

  #[test]
  fn read_seq() {
    let mut stream = TokenStream::new("let x = 10");
    assert_eq!(Ok(Token::Let), stream.take());
    assert_eq!(Ok(Token::Identifier("x")), stream.take());
    assert_eq!(Ok(Token::Equals), stream.take());
    assert_eq!(Ok(Token::Integer(10)), stream.take());
    assert_eq!(Ok(Token::EOF), stream.take());
  }
}

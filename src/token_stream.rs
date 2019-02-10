use crate::char_stream::CharStream;
use crate::parse_utils;
use crate::tokens::Token;

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
  lookahead: Option<(Token<'a>, usize)>,
}

impl<'a> TokenStream<'a> {
  pub fn new(src: &'a str) -> TokenStream<'a> {
    TokenStream {
      stream: CharStream::from_str(src),
      lookahead: None,
    }
  }

  fn read_keyword_or_identifier(&mut self) -> LexerResult<Token<'a>> {
    let keyword_or_identifier = self.stream.take_until(parse_utils::is_whitespace);
    match keyword_or_identifier {
      "const" => Ok(Token::Const),
      "let" => Ok(Token::Let),
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

  fn read_token(&mut self) -> LexerResult<Token<'a>> {
    use Token::*;

    self.stream.skip_whitespace();

    if self.stream.remaining() == 0 {
      return Ok(Token::EOF);
    }

    let offset = self.byte_offset();
    let fst = self
      .stream
      .peek()
      .ok_or_else(|| LexerErrorCtx(offset, LexerError::UnexpectedEof))?;

    match fst {
      '(' => {
        self.stream.advance();
        Ok(LParen)
      }
      ')' => {
        self.stream.advance();
        Ok(RParen)
      }
      '=' => {
        self.stream.advance();
        Ok(Equals)
      }
      ';' => {
        self.stream.advance();
        Ok(Semicolon)
      }
      '0'..='9' => self.read_number(),
      _ => self.read_keyword_or_identifier(),
    }
  }

  pub fn peek_token(&mut self) -> LexerResult<&Token> {
    if self.lookahead.is_none() {
      let offset = self.byte_offset();
      self.lookahead = self.read_token().ok().map(|x| (x, offset));
    }

    let offset = self.byte_offset();
    self
      .lookahead
      .as_ref()
      .map(|x| &x.0)
      .ok_or_else(|| LexerErrorCtx(offset, LexerError::UnexpectedEof))
  }

  pub fn take_token(&mut self) -> LexerResult<Token> {
    if self.lookahead.is_some() {
      let mut token = None;
      std::mem::swap(&mut token, &mut self.lookahead);
      Ok(token.map(|x| x.0).unwrap())
    } else {
      self.read_token()
    }
  }

  pub fn byte_offset(&self) -> usize {
    self.stream.byte_offset()
  }
}

#[cfg(test)]
mod char_stream_tests {
  use super::CharStream;

  #[test]
  fn take_one_empty() {
    let mut stream = CharStream::from_str("");
    assert_eq!(None, stream.take());
  }

  #[test]
  fn take_one_twice() {
    let mut stream = CharStream::from_str("ab");
    assert_eq!(Some('a'), stream.take());
    assert_eq!(Some('b'), stream.take());
  }

  #[test]
  fn take_one_unicode() {
    let mut stream = CharStream::from_str("乇乂丅尺卂 丅卄工匚匚");
    assert_eq!(Some('乇'), stream.take());
    assert_eq!(Some('乂'), stream.take());
  }

  #[test]
  fn take_until_whitespace() {
    let mut stream = CharStream::from_str("abc def");
    let abc = stream.take_until(|c| c == ' ');
    assert_eq!("abc", abc);
    let remaining = stream.take_until(|_| false);
    assert_eq!(" def", remaining);
  }

  #[test]
  fn take_until_all() {
    let mut stream = CharStream::from_str("AAA");
    let aaa = stream.take_until(|c| c != 'A');
    assert_eq!("AAA", aaa);
    assert_eq!(None, stream.take());
  }

  #[test]
  fn skip_while_all() {
    let mut stream = CharStream::from_str("aaaa");
    stream.skip_while(|c| c == 'a');
    assert_eq!(None, stream.take());
  }

  #[test]
  fn skip_until_all() {
    let mut stream = CharStream::from_str("aaaa");
    stream.skip_until(|c| c == 'b');
    assert_eq!(None, stream.take());
  }
}

#[cfg(test)]
mod token_stream_tests {
  use super::{Token, TokenStream};

  #[test]
  fn read_seq() {
    let mut stream = TokenStream::new("const x = 10");
    assert_eq!(Ok(Token::Const), stream.read_token());
    assert_eq!(Ok(Token::Identifier("x")), stream.read_token());
    assert_eq!(Ok(Token::Equals), stream.read_token());
    assert_eq!(Ok(Token::Integer(10)), stream.read_token());
    assert_eq!(Ok(Token::EOF), stream.read_token());
  }
}

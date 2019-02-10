use crate::tokens::Token;

enum ReadTokenError {
  UnknownToken(String),
  UnexpectedEof,
}

enum ConsumeTokenError {
  ReadTokenError,
}

pub struct CharStream<'a> {
  start_offset: usize,
  data: &'a str,
}

impl<'a> CharStream<'a> {
  pub fn from_str(data: &'a str) -> CharStream<'a> {
    CharStream {
      data,
      start_offset: data.as_ptr() as usize,
    }
  }

  fn has_next(&self) -> bool {
    self.data.len() > 0
  }

  pub fn peek(&self) -> Option<char> {
    if self.data.len() == 0 {
      return None;
    }

    self.data.chars().nth(0)
  }

  pub fn advance(&mut self) {
    if self.data.len() > 0 {
      let offset = self.data.chars().nth(0).unwrap().len_utf8();
      self.data = &self.data[offset..];
    }
  }

  pub fn take(&mut self) -> Option<char> {
    if self.data.len() == 0 {
      return None;
    }

    if let Some(ch) = self.data.chars().nth(0) {
      let new_offset = ch.len_utf8();
      self.data = &self.data[new_offset..];
      Some(ch)
    } else {
      unsafe {
        std::hint::unreachable_unchecked();
      }
    }
  }

  pub fn take_until(&mut self, predicate: impl Fn(char) -> bool) -> &'a str {
    let last = self.data.find(predicate);

    match last {
      None => {
        let mut result = "";
        std::mem::swap(&mut result, &mut self.data);
        result
      }
      Some(n) => {
        let (result, remaining) = self.data.split_at(n);
        self.data = remaining;
        result
      }
    }
  }

  pub fn take_while(&mut self, predicate: impl Fn(char) -> bool) -> &'a str {
    self.take_until(|x| !predicate(x))
  }

  pub fn skip_until(&mut self, predicate: impl Fn(char) -> bool) {
    self.take_until(predicate);
  }

  pub fn skip_while(&mut self, predicate: impl Fn(char) -> bool) {
    self.take_while(predicate);
  }

  pub fn skip_whitespace(&mut self) {
    self.skip_while(parse_utils::is_whitespace);
  }

  pub fn byte_offset(&self) -> usize {
    (self.data.as_ptr() as usize)
      .checked_sub(self.start_offset)
      .unwrap()
  }
}

pub struct TokenStream<'a> {
  stream: CharStream<'a>,
  lookahead: Option<Token<'a>>,
}

mod parse_utils {
  pub fn is_whitespace(ch: char) -> bool {
    ch == ' '
  }
}

macro_rules! parse_lexeme {
  (!CASE $stream: expr, _ => $fallback: expr) => { $fallback };
  (!CASE $stream: expr, $ch: pat => $token: expr) => { { $stream.advance(); Some($token) } };
  ($stream: expr, { $($pattern: tt: $result: expr),+ }) => {
    {
      let fst = $stream.peek()?;
      match fst {
        $( $pattern => parse_lexeme!(!CASE $stream, $pattern => $result)),+
      }
    }
  };
}

impl<'a> TokenStream<'a> {
  pub fn new(src: &'a str) -> TokenStream<'a> {
    TokenStream {
      stream: CharStream::from_str(src),
      lookahead: None,
    }
  }

  fn read_keyword_or_identifier(&mut self) -> Option<Token<'a>> {
    let keyword_or_identifier = self.stream.take_until(parse_utils::is_whitespace);
    match keyword_or_identifier {
      "const" => Some(Token::Const),
      "let" => Some(Token::Let),
      otherwise => Some(Token::Identifier(otherwise)),
    }
  }

  fn read_number(&mut self) -> Option<Token<'a>> {
    let chars = self.stream.take_while(|c| c.is_digit(10));
    let parsed = chars.parse().ok()?;
    Some(Token::Integer(parsed))
  }

  fn read_token(&mut self) -> Option<Token<'a>> {
    use Token::*;

    self.stream.skip_whitespace();

    let fst = self.stream.peek()?;

    match fst {
      '(' => {
        self.stream.advance();
        Some(Token::LParen)
      }
      ')' => {
        self.stream.advance();
        Some(Token::RParen)
      }
      '=' => {
        self.stream.advance();
        Some(Token::Equals)
      }
      ';' => {
        self.stream.advance();
        Some(Token::Semicolon)
      }
      '0'..='9' => self.read_number(),
      _ => self.read_keyword_or_identifier(),
    }
  }

  pub fn peek_token(&mut self) -> Option<&Token> {
    if self.lookahead.is_none() {
      self.lookahead = self.read_token();
    }

    self.lookahead.as_ref()
  }

  pub fn take_token(&mut self) -> Option<Token> {
    if self.lookahead.is_some() {
      let mut token = None;
      std::mem::swap(&mut token, &mut self.lookahead);
      token
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
    assert_eq!(Some(Token::Const), stream.read_token());
    assert_eq!(Some(Token::Identifier("x")), stream.read_token());
    assert_eq!(Some(Token::Equals), stream.read_token());
    assert_eq!(Some(Token::Integer(10)), stream.read_token());
    assert_eq!(None, stream.read_token());
  }
}

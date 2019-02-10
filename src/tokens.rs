#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token<'a> {
  Const,
  Let,
  Equals,
  LParen,
  RParen,
  Semicolon,
  Identifier(&'a str),
  Integer(i64),
  EOF,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
  Const,
  Let,
  Equals,
  LParen,
  RParen,
  Semicolon,
  Identifier,
  Integer,
  EOF,
}

impl<'a> Token<'a> {
  pub fn to_kind(&self) -> TokenKind {
    match self {
      Token::Const => TokenKind::Const,
      Token::Let => TokenKind::Let,
      Token::Equals => TokenKind::Equals,
      Token::LParen => TokenKind::LParen,
      Token::RParen => TokenKind::RParen,
      Token::Semicolon => TokenKind::Semicolon,
      Token::Identifier(_) => TokenKind::Identifier,
      Token::Integer(_) => TokenKind::Integer,
      Token::EOF => TokenKind::EOF,
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OperatorToken {
  Plus,
  Minus,
  Asterisk,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token<'a> {
  Let,
  Mut,
  Equals,
  LParen,
  RParen,
  LBrace,
  RBrace,
  Colon,
  Semicolon,
  Identifier(&'a str),
  Integer(i128),
  Operator(OperatorToken),
  EOF,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
  Let,
  Mut,
  Equals,
  LParen,
  RParen,
  LBrace,
  RBrace,
  Colon,
  Semicolon,
  Identifier,
  Integer,
  Operator,
  EOF,
}

impl<'a> Token<'a> {
  pub fn to_kind(&self) -> TokenKind {
    match self {
      Token::Let => TokenKind::Let,
      Token::Mut => TokenKind::Mut,
      Token::Equals => TokenKind::Equals,
      Token::LParen => TokenKind::LParen,
      Token::RParen => TokenKind::RParen,
      Token::LBrace => TokenKind::LBrace,
      Token::RBrace => TokenKind::RBrace,
      Token::Colon => TokenKind::Colon,
      Token::Semicolon => TokenKind::Semicolon,
      Token::Identifier(_) => TokenKind::Identifier,
      Token::Integer(_) => TokenKind::Integer,
      Token::Operator(_) => TokenKind::Operator,
      Token::EOF => TokenKind::EOF,
    }
  }
}

pub fn is_whitespace(ch: char) -> bool {
  ch == ' ' || ch == '\r' || ch == '\n'
}

pub fn is_valid_identifier_first(ch: char) -> bool {
  match ch {
    'A'..='Z' | 'a'..='z' | '_' => true,
    _ => false,
  }
}

pub fn is_valid_in_identifier(ch: char) -> bool {
  match ch {
    '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' => true,
    _ => false,
  }
}

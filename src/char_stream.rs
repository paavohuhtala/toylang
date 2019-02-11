use crate::parse_utils;

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

  pub fn remaining(&self) -> usize {
    self.data.len()
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

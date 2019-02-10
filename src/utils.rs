pub trait ResultExt<T, A> {
  fn err_into<B>(self) -> Result<T, B>
  where
    A: Into<B>;

  fn with_err<B>(self, err: B) -> Result<T, B>;
}

impl<T, A> ResultExt<T, A> for Result<T, A> {
  fn err_into<B>(self) -> Result<T, B>
  where
    A: Into<B>,
  {
    self.map_err(|x| x.into())
  }

  fn with_err<B>(self, err: B) -> Result<T, B> {
    self.map_err(|_| err)
  }
}

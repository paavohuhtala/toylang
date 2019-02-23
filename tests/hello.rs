use toylang::eval;

#[test]
pub fn hello_world() {
  assert_eq!(Ok(None), eval("let x = 10;"));
}

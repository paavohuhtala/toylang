pub mod ast;
pub mod ast_common;
pub mod char_stream;
pub mod interpreter;
pub mod rast;
pub mod parse_utils;
pub mod parser;
pub mod semantic;
pub mod token_stream;
pub mod tokens;
pub mod type_checker;
pub mod utils;

use crate::parser::ParseErrorCtx;
use crate::semantic::SemanticErrorCtx;
use crate::type_checker::TypeErrorCtx;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalError {
  ParseError(ParseErrorCtx),
  SemanticError(SemanticErrorCtx),
  TypeError(TypeErrorCtx),
}

impl Into<EvalError> for ParseErrorCtx {
  fn into(self) -> EvalError {
    EvalError::ParseError(self)
  }
}

impl Into<EvalError> for SemanticErrorCtx {
  fn into(self) -> EvalError {
    EvalError::SemanticError(self)
  }
}

impl Into<EvalError> for TypeErrorCtx {
  fn into(self) -> EvalError {
    EvalError::TypeError(self)
  }
}

pub fn eval(src: &str) -> Result<Option<interpreter::Value>, EvalError> {
  use crate::interpreter::Interpreter;
  use crate::parser::Parser;
  use crate::semantic::transform_program;
  use crate::token_stream::TokenStream;
  use crate::type_checker::visit_program;
  use crate::utils::ResultExt;

  let mut token_stream = TokenStream::new(src);
  let mut parser = Parser::new(&mut token_stream);
  let program = parser.parse_program().err_into()?;

  let (mut ctx, mut program) = transform_program(program).err_into()?;
  visit_program(&mut ctx, &mut program).err_into()?;

  let mut interpreter = Interpreter::new(ctx);
  interpreter.execute_program(&program);
  Ok(None)
}

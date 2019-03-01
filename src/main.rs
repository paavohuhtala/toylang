mod ast;
mod ast_common;
mod char_stream;
mod interpreter;
mod parse_utils;
mod parser;
mod rast;
mod semantic;
mod token_stream;
mod tokens;
mod type_checker;
mod utils;

use std::io::stdin;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::semantic::transform_program;
use crate::token_stream::TokenStream;
use crate::type_checker::visit_program;

fn main() {
  let input = stdin();
  let mut buffer = String::new();
  loop {
    buffer.clear();
    input.read_line(&mut buffer).unwrap();
    let mut token_stream = TokenStream::new(&buffer);
    let mut parser = Parser::new(&mut token_stream);
    let program = parser.parse_program();
    println!("Parsed: {:#?}", program);

    let program = if let Ok(program) = program {
      program
    } else {
      continue;
    };

    let transformed = transform_program(program);
    println!("RAST: {:#?}", transformed);

    if let Ok((mut ctx, mut program)) = transformed {
      match visit_program(&mut ctx, &mut program) {
        Ok(_) => {
          println!("Type checked OK! Locals: {:#?}", ctx.locals);
          let mut interpreter = Interpreter::new(ctx);
          interpreter.execute_program(&program);
          println!("Locals: {:?}", interpreter.locals);
        }
        Err(err) => println!("Err: {:?}", err),
      }
    }
  }
}

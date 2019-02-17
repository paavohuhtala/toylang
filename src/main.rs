mod ast;
mod char_stream;
mod mir;
mod parse_utils;
mod parser;
mod token_stream;
mod tokens;
mod type_checker;
mod utils;

use std::io::stdin;

use crate::ast::StatementCtx;
use crate::mir::{transform_statement, SemanticContext};
use crate::parser::Parser;
use crate::token_stream::TokenStream;

fn main() {
  let input = stdin();
  let mut buffer = String::new();
  loop {
    buffer.clear();
    input.read_line(&mut buffer).unwrap();
    let mut token_stream = TokenStream::new(&buffer);
    let mut parser = Parser::new(&mut token_stream);
    let statement = parser.parse_statement();
    println!("Parsed: {:?}", statement);

    let mut ctx = SemanticContext::new();
    let root_scope = ctx.declare_scope(None);
    let transformed =
      statement.map(|StatementCtx(_, x)| transform_statement(&mut ctx, root_scope, &x));

    println!("MIR: {:?}", transformed);
  }
}

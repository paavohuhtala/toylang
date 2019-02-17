mod ast;
mod char_stream;
mod mir;
mod parse_utils;
mod parser;
mod semantic;
mod token_stream;
mod tokens;
mod type_checker;
mod utils;

use std::io::stdin;

use crate::parser::Parser;
use crate::semantic::transform_program;
use crate::token_stream::TokenStream;

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

    let transformed = program.map(transform_program);
    println!("MIR: {:#?}", transformed);
  }
}

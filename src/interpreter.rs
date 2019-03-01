use std::collections::HashMap;

use crate::ast_common::{BinaryOperator, UnaryOperator};
use crate::rast::{LocalId, RastExpression, RastProgram, RastStatement};
use crate::semantic::SemanticContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
  I32(i32),
  Bool(bool),
}

pub struct Interpreter {
  semantic_ctx: SemanticContext,
  pub locals: HashMap<LocalId, Value>,
}

impl Interpreter {
  pub fn new(semantic_ctx: SemanticContext) -> Interpreter {
    Interpreter {
      semantic_ctx,
      locals: HashMap::new(),
    }
  }

  fn evaluate(&mut self, expression: &RastExpression) -> Value {
    use BinaryOperator::*;
    use RastExpression::*;
    use UnaryOperator::*;
    use Value::*;

    match expression {
      IntegerConstant(i) => I32(*i as i32),
      Local(local_id) => *self.locals.get(local_id).unwrap(),
      UnaryOp(Negate, expr) => {
        if let I32(i) = self.evaluate(&expr.1) {
          I32(-i)
        } else {
          panic!()
        }
      }
      BinaryOp(op, args) => {
        let lhs = self.evaluate(&(args.0).1);
        let rhs = self.evaluate(&(args.1).1);

        match (lhs, op, rhs) {
          (I32(a), Add, I32(b)) => I32(a + b),
          (I32(a), Sub, I32(b)) => I32(a - b),
          (I32(a), Mul, I32(b)) => I32(a * b),
          _ => unreachable!(),
        }
      }
      _ => unreachable!(),
    }
  }

  pub fn execute(&mut self, statement: &RastStatement) -> Option<Value> {
    match statement {
      RastStatement::AssignLocal { local_id, value } => {
        let rhs = self.evaluate(&value.1);
        self.locals.insert(*local_id, rhs);
        None
      }
      RastStatement::Block { inner, .. } => {
        for statement in inner {
          self.execute(&statement.1);
        }
        None
      }
    }
  }

  pub fn execute_program(&mut self, program: &RastProgram) {
    for statement in &program.0 {
      self.execute(&statement.1);
    }
  }
}

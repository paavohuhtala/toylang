use crate::mir::{LocalId, MirExpression, MirProgram, MirStatement};
use crate::semantic::SemanticContext;
use std::collections::HashMap;

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

  fn evaluate(&mut self, expression: &MirExpression) -> Value {
    match expression {
      MirExpression::IntegerConstant(i) => Value::I32(*i as i32),
      MirExpression::Local(local_id) => *self.locals.get(local_id).unwrap(),
      _ => unimplemented!(),
    }
  }

  pub fn execute(&mut self, statement: &MirStatement) -> Option<Value> {
    match statement {
      MirStatement::AssignLocal { local_id, value } => {
        let rhs = self.evaluate(value);
        self.locals.insert(*local_id, rhs);
        None
      }
      MirStatement::Block { inner, .. } => {
        for statement in inner {
          self.execute(statement);
        }
        None
      }
    }
  }

  pub fn execute_program(&mut self, program: &MirProgram) {
    for statement in &program.0 {
      self.execute(statement);
    }
  }
}

use crate::ast::{Expression, ExpressionCtx, Statement, StatementCtx};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ScopeId(usize);

pub struct Scope {
  parent: Option<ScopeId>,
}

pub struct PrimitiveTypes {
  i32_type: Type,
  bool_type: Type,
}

impl Default for PrimitiveTypes {
  fn default() -> PrimitiveTypes {
    PrimitiveTypes {
      i32_type: Type::Primitive(PrimitiveType::I32),
      bool_type: Type::Primitive(PrimitiveType::Bool),
    }
  }
}

pub struct TypeCheckingCtx {
  scopes: HashMap<ScopeId, Scope>,
  types: HashMap<TypeId, Type>,
  named_types: HashMap<String, TypeId>,
  pub primitive: PrimitiveTypes,
}

impl TypeCheckingCtx {
  pub fn new() -> TypeCheckingCtx {
    TypeCheckingCtx {
      scopes: HashMap::new(),
      types: HashMap::new(),
      named_types: HashMap::new(),
      primitive: PrimitiveTypes::default(),
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TypeId(usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveType {
  I32,
  Bool,
}

pub enum Type {
  Named(String),
  Primitive(PrimitiveType),
  Array(TypeId),
}

impl TypeCheckingCtx {
  pub fn resolve_named_type(&self, name: &str) -> Option<&Type> {
    match name {
      "i32" => Some(&self.primitive.i32_type),
      "bool" => Some(&self.primitive.bool_type),
      name => {
        let id = self.named_types.get(name)?;
        self.types.get(id)
      }
    }
  }

  pub fn resolve_type(&self, type_id: TypeId) -> Option<&Type> {
    self.types.get(&type_id)
  }

  pub fn define_alias(&mut self, name: &str, type_id: TypeId) {
    self.named_types.insert(name.to_string(), type_id).unwrap();
  }

  pub fn resolve_expression(&mut self, expression: &ExpressionCtx) -> Option<&Type> {
    match &expression.1 {
      Expression::IntegerLiteral(_) => Some(&self.primitive.i32_type),
      _ => None,
    }
  }

  pub fn visit_statement(&mut self, statement: &StatementCtx) -> bool {
    match &statement.1 {
      Statement::DeclareVariable {
        initial_type,
        initial_value,
        ..
      } => false,
      _ => false,
    }
  }
}

impl Type {
  fn is_assignable_to(&self, other: &Type, ctx: &TypeCheckingCtx) -> bool {
    use self::Type::*;

    match (self, other) {
      (Named(a), Named(b)) => match (ctx.resolve_named_type(a), ctx.resolve_named_type(b)) {
        (Some(a), Some(b)) => a.is_assignable_to(b, ctx),
        _ => false,
      },
      (Primitive(a), Primitive(b)) => a == b,
      (Array(a), Array(b)) => match (ctx.resolve_type(*a), ctx.resolve_type(*b)) {
        (Some(a), Some(b)) => a.is_assignable_to(b, ctx),
        _ => false,
      },
      _ => false,
    }
  }
}

#[cfg(test)]
mod assignability_tests {
  use super::*;

  #[test]
  fn primitive_resolved() {
    let ctx = TypeCheckingCtx::new();

    assert!(ctx
      .primitive
      .i32_type
      .is_assignable_to(&ctx.primitive.i32_type, &ctx))
  }

  #[test]
  fn primitive_named() {
    let ctx = TypeCheckingCtx::new();
    assert!(Type::Named("i32".to_string()).is_assignable_to(&Type::Named("i32".to_string()), &ctx));
  }
}

#![allow(dead_code)]

// RAST (pronounced like rust, of course) stands for Resolved AST
// It represents a higher level AST after name and/or type resolution.

use std::collections::HashSet;

use crate::ast_common::{BinaryOperator, UnaryOperator};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub struct ScopeId(pub(crate) usize);

impl ScopeId {
  pub fn next(&mut self) -> ScopeId {
    let current = self.0;
    self.0 += 1;
    ScopeId(current)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub struct LocalId(pub(crate) usize);

impl LocalId {
  pub fn next(&mut self) -> LocalId {
    let current = self.0;
    self.0 += 1;
    LocalId(current)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TypeRef {
  Primitive(PrimitiveType),
  UserType(UserTypeId),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub struct UserTypeId(pub(crate) usize);

impl UserTypeId {
  pub fn next(&mut self) -> UserTypeId {
    let current = self.0;
    self.0 += 1;
    UserTypeId(current)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveType {
  I32,
  Bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UserTypeDef {
  Array(TypeRef),
}

#[derive(Debug, PartialEq, Eq)]
pub struct UserType {
  pub id: UserTypeId,
  pub type_def: UserTypeDef,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Local {
  pub id: LocalId,
  pub scope_id: ScopeId,
  pub initial_type: Option<TypeRef>,
  pub name: String,
}

#[derive(Debug, PartialEq, Eq)]

pub struct Scope {
  pub id: ScopeId,
  pub parent: Option<ScopeId>,
  pub locals: HashSet<LocalId>,
}

impl Scope {
  pub fn new(id: ScopeId, parent: Option<ScopeId>) -> Scope {
    Scope {
      id,
      parent,
      locals: HashSet::new(),
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RastExpression {
  IntegerConstant(i128),
  Local(LocalId),
  UnaryOp(UnaryOperator, Box<RastExpressionCtx>),
  BinaryOp(BinaryOperator, Box<(RastExpressionCtx, RastExpressionCtx)>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct RastExpressionCtx(pub usize, pub RastExpression);

#[derive(Debug, PartialEq, Eq)]
pub enum RastStatement {
  Block {
    scope_id: ScopeId,
    inner: Vec<RastStatementCtx>,
  },
  AssignLocal {
    local_id: LocalId,
    value: RastExpressionCtx,
  },
}

#[derive(Debug, PartialEq, Eq)]
pub struct RastStatementCtx(pub usize, pub RastStatement);

#[derive(Debug)]
pub struct RastProgram(pub Vec<RastStatementCtx>);

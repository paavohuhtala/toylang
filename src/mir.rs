#![allow(dead_code)]

use crate::ast::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub struct ScopeId(usize);

impl ScopeId {
  pub fn next(&mut self) -> ScopeId {
    let current = self.0;
    self.0 += 1;
    ScopeId(current)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
pub struct LocalId(usize);

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
pub struct UserTypeId(usize);

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
  id: UserTypeId,
  type_def: UserTypeDef,
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
  id: ScopeId,
  parent: Option<ScopeId>,
  locals: HashSet<LocalId>,
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
pub enum MirExpression {
  IntegerConstant(i128),
  Local(LocalId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum MirStatement {
  Block {
    scope_id: ScopeId,
    inner: Vec<MirStatement>,
  },
  AssignLocal {
    local_id: LocalId,
    value: MirExpression,
  },
}

#[derive(Debug)]
pub struct MirProgram(Vec<MirStatement>);

#[derive(Debug, PartialEq, Eq)]
pub struct SemanticContext {
  user_types: HashMap<UserTypeId, UserType>,
  scopes: HashMap<ScopeId, Scope>,
  locals: HashMap<LocalId, Local>,
  next_scope_id: ScopeId,
  next_user_type_id: UserTypeId,
  next_local_id: LocalId,
}

impl SemanticContext {
  pub fn new() -> SemanticContext {
    SemanticContext {
      user_types: HashMap::new(),
      scopes: HashMap::new(),
      locals: HashMap::new(),
      next_scope_id: ScopeId::default(),
      next_user_type_id: UserTypeId::default(),
      next_local_id: LocalId::default(),
    }
  }

  pub fn declare_local(
    &mut self,
    scope_id: ScopeId,
    name: String,
    initial_type: Option<TypeRef>,
  ) -> LocalId {
    let id = self.next_local_id.next();

    let scope = self.scopes.get_mut(&scope_id).unwrap();
    scope.locals.insert(id);

    self.locals.insert(
      id,
      Local {
        id,
        scope_id,
        name,
        initial_type,
      },
    );

    id
  }

  pub fn declare_type(&mut self, type_: UserType) -> TypeRef {
    let id = self.next_user_type_id.next();
    self.user_types.insert(id, type_);
    TypeRef::UserType(id)
  }

  pub fn declare_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
    let id = self.next_scope_id.next();
    let scope = Scope::new(id, parent);
    self.scopes.insert(id, scope);
    id
  }

  pub fn resolve_named_type(&self, name: &str) -> Option<TypeRef> {
    match name {
      "i32" => Some(TypeRef::Primitive(PrimitiveType::I32)),
      "bool" => Some(TypeRef::Primitive(PrimitiveType::Bool)),
      _ => None,
    }
  }

  pub fn resolve_scope(&self, scope_id: ScopeId) -> &Scope {
    self.scopes.get(&scope_id).unwrap()
  }

  pub fn resolve_scope_mut(&mut self, scope_id: ScopeId) -> &mut Scope {
    self.scopes.get_mut(&scope_id).unwrap()
  }

  pub fn is_local_within_scope(&self, mut scope_id: ScopeId, local_id: LocalId) -> bool {
    let local = self.locals.get(&local_id).unwrap();
    loop {
      let scope = self.resolve_scope(scope_id);

      if scope.id == local.scope_id {
        return true;
      }

      if let Some(parent) = scope.parent {
        scope_id = parent;
      } else {
        return false;
      }
    }
  }

  pub fn resolve_named_local(&self, scope_id: ScopeId, name: &str) -> Option<LocalId> {
    self
      .locals
      .values()
      .find(|x| x.name == name)
      .map(|x| x.id)
      .filter(|id| self.is_local_within_scope(scope_id, *id))
  }

  pub fn resolve_local(&self, scope_id: ScopeId, local_id: LocalId) -> Option<&Local> {
    self
      .locals
      .get(&local_id)
      .filter(|local| self.is_local_within_scope(scope_id, local.id))
  }

  pub fn resolve_local_mut(&mut self, scope_id: ScopeId, local_id: LocalId) -> Option<&mut Local> {
    if self.is_local_within_scope(scope_id, local_id) {
      self.locals.get_mut(&local_id)
    } else {
      None
    }
  }
}

pub fn transform_expression(
  ctx: &mut SemanticContext,
  scope_id: ScopeId,
  expression: &Expression,
) -> MirExpression {
  match expression {
    Expression::IntegerConstant(x) => MirExpression::IntegerConstant(*x),
    Expression::Local(local) => {
      let local_id = ctx.resolve_named_local(scope_id, local).unwrap();
      MirExpression::Local(local_id)
    }
    _ => panic!(),
  }
}

pub fn transform_statement(
  ctx: &mut SemanticContext,
  scope_id: ScopeId,
  statement: &Statement,
) -> MirStatement {
  // let scope = ctx.scopes.get(&scope_id).unwrap();
  match statement {
    Statement::Block { inner } => {
      let scope_id = ctx.declare_scope(Some(scope_id));
      MirStatement::Block {
        scope_id,
        inner: inner
          .iter()
          .map(|StatementCtx(_, statement)| transform_statement(ctx, scope_id, statement))
          .collect(),
      }
    }
    Statement::AssignLocal { local, value } => {
      let local_id = ctx.resolve_named_local(scope_id, &local.1).unwrap();
      MirStatement::AssignLocal {
        local_id,
        value: transform_expression(ctx, scope_id, &value.1),
      }
    }
    Statement::DeclareVariable {
      name,
      initial_type,
      initial_value,
      ..
    } => {
      let initial_type = initial_type
        .as_ref()
        .and_then(|x| ctx.resolve_named_type(&x.1));
      let local_id = ctx.declare_local(scope_id, name.1.clone(), initial_type);
      let value = transform_expression(ctx, scope_id, &initial_value.1);

      MirStatement::AssignLocal { local_id, value }
    }
  }
}

pub fn transform_program(Program(statements): Program) -> (SemanticContext, MirProgram) {
  let mut ctx = SemanticContext::new();
  let root_scope = ctx.declare_scope(None);

  let mut transformed_statements = Vec::new();
  for statement in statements {
    transformed_statements.push(transform_statement(&mut ctx, root_scope, &statement.1));
  }

  (ctx, MirProgram(transformed_statements))
}

#[cfg(test)]
mod mir_transformer_tests {
  use super::*;

  #[test]
  fn transform_assignment() {
    let mut ctx = SemanticContext::new();

    let ast = Statement::DeclareVariable {
      name: IdentifierCtx(0, "x".to_string()),
      is_mutable: false,
      initial_type: Some(IdentifierCtx(0, "i32".to_string())),
      initial_value: ExpressionCtx(0, Expression::IntegerConstant(32)),
    };

    let scope_id = ctx.declare_scope(None);
    let transformed = transform_statement(&mut ctx, scope_id, &ast);

    let scope = ctx.resolve_scope(scope_id);
    assert_eq!(ScopeId(0), scope.id);
    assert_eq!(
      vec![LocalId(0)],
      scope.locals.iter().cloned().collect::<Vec<LocalId>>()
    );

    let local = ctx.resolve_local(scope_id, LocalId(0)).unwrap();
    assert_eq!(local.id, LocalId(0));

    assert_eq!(
      transformed,
      MirStatement::AssignLocal {
        local_id: LocalId(0),
        value: MirExpression::IntegerConstant(32)
      }
    );
  }
}

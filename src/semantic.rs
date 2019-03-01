use crate::ast::*;
use crate::rast::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct SemanticContext {
  user_types: HashMap<UserTypeId, UserType>,
  scopes: HashMap<ScopeId, Scope>,
  pub locals: HashMap<LocalId, Local>,
  next_scope_id: ScopeId,
  next_user_type_id: UserTypeId,
  next_local_id: LocalId,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SemanticError {
  UnknownType { name: String },
  UnknownLocal { name: String },
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemanticErrorCtx(pub usize, pub SemanticError);

pub type SemanticResult<T> = Result<T, SemanticErrorCtx>;

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

  pub fn resolve_named_type(
    &self,
    IdentifierCtx(pos, identifier): &IdentifierCtx,
  ) -> SemanticResult<TypeRef> {
    match identifier.as_str() {
      "i32" => Ok(TypeRef::Primitive(PrimitiveType::I32)),
      "bool" => Ok(TypeRef::Primitive(PrimitiveType::Bool)),
      _ => Err(SemanticErrorCtx(
        *pos,
        SemanticError::UnknownType {
          name: identifier.clone(),
        },
      )),
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
  expression: &ExpressionCtx,
) -> SemanticResult<RastExpressionCtx> {
  let ExpressionCtx(pos, expression) = expression;
  match expression {
    Expression::IntegerConstant(x) => {
      Ok(RastExpressionCtx(*pos, RastExpression::IntegerConstant(*x)))
    }
    Expression::Local(local) => match ctx.resolve_named_local(scope_id, local) {
      Some(local_id) => Ok(RastExpressionCtx(*pos, RastExpression::Local(local_id))),
      None => Err(SemanticErrorCtx(
        *pos,
        SemanticError::UnknownLocal {
          name: local.to_string(),
        },
      )),
    },
    Expression::UnaryOp(op, arg) => {
      let value = transform_expression(ctx, scope_id, arg)?;
      Ok(RastExpressionCtx(
        *pos,
        RastExpression::UnaryOp(*op, Box::new(value)),
      ))
    }
    Expression::BinaryOp(op, args) => {
      let lhs = transform_expression(ctx, scope_id, &args.0)?;
      let rhs = transform_expression(ctx, scope_id, &args.1)?;
      Ok(RastExpressionCtx(
        *pos,
        RastExpression::BinaryOp(*op, Box::new((lhs, rhs))),
      ))
    }
  }
}

pub fn transform_statement(
  ctx: &mut SemanticContext,
  scope_id: ScopeId,
  statement: &StatementCtx,
) -> SemanticResult<RastStatementCtx> {
  let StatementCtx(pos, statement) = statement;
  match statement {
    Statement::Block { inner } => {
      let scope_id = ctx.declare_scope(Some(scope_id));
      let inner: Result<_, _> = inner
        .iter()
        .map(|statement| transform_statement(ctx, scope_id, statement))
        .collect();
      let inner = inner?;

      Ok(RastStatementCtx(
        *pos,
        RastStatement::Block { scope_id, inner },
      ))
    }
    Statement::AssignLocal { local, value } => {
      let IdentifierCtx(pos, identifier) = local;
      match ctx.resolve_named_local(scope_id, identifier) {
        Some(local_id) => Ok(RastStatementCtx(
          *pos,
          RastStatement::AssignLocal {
            local_id,
            value: transform_expression(ctx, scope_id, value)?,
          },
        )),
        None => Err(SemanticErrorCtx(
          *pos,
          SemanticError::UnknownLocal {
            name: identifier.to_string(),
          },
        )),
      }
    }
    Statement::DeclareVariable {
      name,
      initial_type,
      initial_value,
      ..
    } => {
      let initial_type = match initial_type {
        Some(x) => Some(ctx.resolve_named_type(x)?),
        None => None,
      };
      let local_id = ctx.declare_local(scope_id, name.1.clone(), initial_type);
      let value = transform_expression(ctx, scope_id, initial_value)?;

      Ok(RastStatementCtx(
        *pos,
        RastStatement::AssignLocal { local_id, value },
      ))
    }
  }
}

pub fn transform_program(
  Program(statements): Program,
) -> SemanticResult<(SemanticContext, RastProgram)> {
  let mut ctx = SemanticContext::new();
  let root_scope = ctx.declare_scope(None);

  let mut transformed_statements = Vec::new();
  for statement in statements {
    transformed_statements.push(transform_statement(&mut ctx, root_scope, &statement)?);
  }

  Ok((ctx, RastProgram(transformed_statements)))
}

#[cfg(test)]
mod rast_transformer_tests {
  use super::*;

  #[test]
  fn transform_assignment() {
    let mut ctx = SemanticContext::new();

    let ast = StatementCtx(
      0,
      Statement::DeclareVariable {
        name: IdentifierCtx(0, "x".to_string()),
        is_mutable: false,
        initial_type: Some(IdentifierCtx(0, "i32".to_string())),
        initial_value: ExpressionCtx(0, Expression::IntegerConstant(32)),
      },
    );

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
      Ok(RastStatementCtx(
        0,
        RastStatement::AssignLocal {
          local_id: LocalId(0),
          value: RastExpressionCtx(0, RastExpression::IntegerConstant(32))
        }
      ))
    );
  }
}

#![allow(dead_code)]

use crate::mir::ScopeId;
use crate::mir::SemanticContext;
use crate::mir::{MirExpression, MirStatement, PrimitiveType, TypeRef};

pub enum TypeError {
  NotAssignable { target: TypeRef, x: TypeRef },
}

pub struct TypeErrorCtx(usize, TypeError);

pub type TypeResult<T> = Result<T, TypeErrorCtx>;

pub fn are_equal(ctx: &mut SemanticContext, a: TypeRef, b: TypeRef) -> bool {
  use self::TypeRef::*;
  match (a, b) {
    (Primitive(a), Primitive(b)) => a == b,
    _ => false,
  }
}

pub fn is_assignable(ctx: &mut SemanticContext, a: TypeRef, b: TypeRef) -> bool {
  use self::TypeRef::*;
  match (a, b) {
    (Primitive(a), Primitive(b)) => a == b,
    _ => false,
  }
}

pub fn resolve_expression(
  ctx: &mut SemanticContext,
  scope_id: ScopeId,
  expression: &MirExpression,
) -> Option<TypeRef> {
  match expression {
    MirExpression::IntegerConstant(_) => Some(TypeRef::Primitive(PrimitiveType::I32)),
    _ => None,
  }
}

pub fn visit_statement(
  ctx: &mut SemanticContext,
  scope_id: ScopeId,
  statement: &mut MirStatement,
) -> TypeResult<()> {
  match statement {
    MirStatement::AssignLocal {
      local_id, value, ..
    } => {
      let value_type = resolve_expression(ctx, scope_id, value).unwrap();
      let local = ctx.resolve_local_mut(scope_id, *local_id).unwrap();

      if local.initial_type == None {
        local.initial_type = Some(value_type);
      } else if let Some(annotated_type) = local.initial_type {
        if !is_assignable(ctx, annotated_type, value_type) {
          return Err(TypeErrorCtx(
            0,
            TypeError::NotAssignable {
              target: annotated_type,
              x: value_type,
            },
          ));
        }
      }

      Ok(())
    }
    MirStatement::Block { scope_id, inner } => {
      for statement in inner {
        visit_statement(ctx, *scope_id, statement)?;
      }

      Ok(())
    }
    _ => panic!(),
  }
}

#[cfg(test)]
mod assignability_tests {
  use super::*;

  #[test]
  fn primitive_resolved() {
    let mut ctx = SemanticContext::new();
    assert!(is_assignable(
      &mut ctx,
      TypeRef::Primitive(PrimitiveType::I32),
      TypeRef::Primitive(PrimitiveType::I32)
    ));
  }
}

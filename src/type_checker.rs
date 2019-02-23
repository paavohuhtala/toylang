#![allow(dead_code)]

use crate::ast_common::{BinaryOperator, UnaryOperator};
use crate::mir::{
  LocalId, MirExpression, MirExpressionCtx, MirProgram, MirStatement, MirStatementCtx,
  PrimitiveType, ScopeId, TypeRef,
};
use crate::semantic::SemanticContext;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeError {
  NotAssignable {
    target: TypeRef,
    x: TypeRef,
  },
  InvalidUnaryOpArg {
    op: UnaryOperator,
    x: TypeRef,
  },
  InvalidBinaryOpArgs {
    op: BinaryOperator,
    lhs: TypeRef,
    rhs: TypeRef,
  },
  UntypedLocal {
    local_id: LocalId,
  },
}

#[derive(Debug, PartialEq, Eq)]
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
  expression: &MirExpressionCtx,
) -> TypeResult<TypeRef> {
  use BinaryOperator::*;
  use MirExpression::*;
  use PrimitiveType::*;
  use TypeRef::*;
  use UnaryOperator::*;

  let MirExpressionCtx(pos, expression) = expression;

  match expression {
    IntegerConstant(_) => Ok(Primitive(I32)),
    &Local(local_id) => {
      let local = ctx.resolve_local(scope_id, local_id).unwrap();
      local
        .initial_type
        .ok_or_else(|| TypeErrorCtx(*pos, TypeError::UntypedLocal { local_id }))
    }
    UnaryOp(op, x) => {
      let x_type = resolve_expression(ctx, scope_id, x)?;
      match (*op, x_type) {
        (Negate, Primitive(I32)) => Ok(Primitive(I32)),
        _ => Err(TypeErrorCtx(
          *pos,
          TypeError::InvalidUnaryOpArg { op: *op, x: x_type },
        )),
      }
    }
    BinaryOp(op, args) => {
      let lhs_type = resolve_expression(ctx, scope_id, &args.0)?;
      let rhs_type = resolve_expression(ctx, scope_id, &args.1)?;

      match (lhs_type, *op, rhs_type) {
        (Primitive(I32), Add, Primitive(I32))
        | (Primitive(I32), Sub, Primitive(I32))
        | (Primitive(I32), Mul, Primitive(I32)) => Ok(Primitive(I32)),
        _ => Err(TypeErrorCtx(
          *pos,
          TypeError::InvalidBinaryOpArgs {
            op: *op,
            lhs: lhs_type,
            rhs: rhs_type,
          },
        )),
      }
    }
    _ => unimplemented!(),
  }
}

pub fn visit_statement(
  ctx: &mut SemanticContext,
  scope_id: ScopeId,
  statement: &mut MirStatementCtx,
) -> TypeResult<()> {
  let MirStatementCtx(pos, statement) = statement;

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
            *pos,
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

pub fn visit_program(ctx: &mut SemanticContext, program: &mut MirProgram) -> TypeResult<()> {
  for statement in &mut program.0 {
    visit_statement(ctx, ScopeId(0), statement)?;
  }

  Ok(())
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

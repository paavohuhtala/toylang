#![allow(dead_code)]

use crate::ast_common::{BinaryOperator, UnaryOperator};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
  IntegerConstant(i128),
  Local(String),
  BinaryOp(BinaryOperator, Box<(ExpressionCtx, ExpressionCtx)>),
  UnaryOp(UnaryOperator, Box<ExpressionCtx>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExpressionCtx(pub usize, pub Expression);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IdentifierCtx(pub usize, pub String);

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Statement {
  DeclareVariable {
    name: IdentifierCtx,
    is_mutable: bool,
    initial_type: Option<IdentifierCtx>,
    initial_value: ExpressionCtx,
  },
  AssignLocal {
    local: IdentifierCtx,
    value: ExpressionCtx,
  },
  Block {
    inner: Vec<StatementCtx>,
  },
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StatementCtx(pub usize, pub Statement);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Block(pub Vec<StatementCtx>);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Program(pub Vec<StatementCtx>);

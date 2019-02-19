#![allow(dead_code)]

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BinaryOperator {
  Add,
  Mul,
  Sub,
  Equals,
}

impl BinaryOperator {
  pub fn get_precedence(&self) -> i32 {
    match self {
      BinaryOperator::Mul => 3,
      BinaryOperator::Add | BinaryOperator::Sub => 2,
      BinaryOperator::Equals => 0,
    }
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum UnaryOperator {
  Negate,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Operator {
  Binary(BinaryOperator),
  Unary(UnaryOperator),
}

impl Operator {
  pub fn get_precedence(&self) -> i32 {
    match self {
      Operator::Binary(BinaryOperator::Mul) => 3,
      Operator::Binary(BinaryOperator::Add) | Operator::Binary(BinaryOperator::Sub) => 2,
      Operator::Unary(UnaryOperator::Negate) => 1,
      Operator::Binary(BinaryOperator::Equals) => 0,
    }
  }
}

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

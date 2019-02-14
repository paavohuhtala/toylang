#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BinaryOperator {
  Add,
  Sub,
  Equals,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
  IntegerLiteral(i64),
  BinaryExpression(BinaryOperator, Box<(ExpressionCtx, ExpressionCtx)>),
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
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StatementCtx(pub usize, pub Statement);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Block(pub Vec<StatementCtx>);

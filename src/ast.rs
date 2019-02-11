#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AstId(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BinaryOperator {
  Add,
  Sub,
  Equals,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
  IntegerLiteral(i64),
  BinaryExpression(BinaryOperator, Box<(Expression, Expression)>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Statement {
  DeclareVariable {
    name: String,
    is_mutable: bool,
    initial_value: Expression,
  },
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Block(pub Vec<Statement>);

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
#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub function_definition: FunctionDefinition,
}

pub type Identifier = String;
#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub body: Statement,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Constant(i32),
    Unary(UnaryOperation, Box<Expression>),
    Binary(BinaryOperation, Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperation {
    Complement,
    Negate,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

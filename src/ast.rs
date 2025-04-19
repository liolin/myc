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
    Return(Return),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Return {
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Constant(i32),
}

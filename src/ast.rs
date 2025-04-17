#[derive(Debug)]
pub struct Program {
    pub function_definition: FunctionDefinition,
}

pub type Identifier = String;
#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub body: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return(Return),
}

#[derive(Debug)]
pub struct Return {
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
}

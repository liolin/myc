use crate::ast;

pub struct Program {
    pub function_definition: FunctionDefinition,
}

pub type Identifier = String;
pub struct FunctionDefinition {
    pub name: Identifier,
    pub instructions: Vec<Instruction>,
}

pub enum Instruction {
    Mov(Mov),
    Ret,
}

pub struct Mov {
    pub src: Operand,
    pub dst: Operand,
}

impl Mov {
    pub fn new(src: Operand, dst: Operand) -> Self {
        Self { src, dst }
    }
}

pub enum Operand {
    Imm(i32),
    Register,
}

pub fn assembly(program: ast::Program) -> Program {
    Program {
        function_definition: function_definition(program.function_definition),
    }
}

fn function_definition(function: ast::FunctionDefinition) -> FunctionDefinition {
    FunctionDefinition {
        name: function.name,
        instructions: instructions(function.body),
    }
}

fn instructions(statement: ast::Statement) -> Vec<Instruction> {
    match statement {
        ast::Statement::Return(r) => return_(r),
    }
}

fn return_(statement: ast::Return) -> Vec<Instruction> {
    let exp = expression(statement.expression);
    vec![
        Instruction::Mov(Mov::new(exp, Operand::Register)),
        Instruction::Ret,
    ]
}

fn expression(expression: ast::Expression) -> Operand {
    match expression {
        ast::Expression::Constant(i) => Operand::Imm(i),
    }
}

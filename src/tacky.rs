use crate::ast;

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub function: Function,
}

pub type Identifier = String;
#[derive(Debug, PartialEq, Eq)]
pub struct Function {
    pub name: Identifier,
    pub body: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Return(Value),
    Unary {
        operator: UnaryOperator,
        src: Value,
        dst: Value,
    },
    Binary {
        operator: BinaryOperator,
        left: Value,
        right: Value,
        dst: Value,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Constant(i32),
    Var(Identifier),
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

impl From<ast::UnaryOperation> for UnaryOperator {
    fn from(value: ast::UnaryOperation) -> Self {
        match value {
            ast::UnaryOperation::Complement => Self::Complement,
            ast::UnaryOperation::Negate => Self::Negate,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

impl From<ast::BinaryOperation> for BinaryOperator {
    fn from(value: ast::BinaryOperation) -> Self {
        match value {
            ast::BinaryOperation::Add => Self::Add,
            ast::BinaryOperation::Subtract => Self::Subtract,
            ast::BinaryOperation::Multiply => Self::Multiply,
            ast::BinaryOperation::Divide => Self::Divide,
            ast::BinaryOperation::Remainder => Self::Remainder,
        }
    }
}

pub fn tacky(ast: ast::Program) -> Program {
    let mut t = TackyGen::new();
    t.program(ast)
}

pub struct TackyGen {
    counter: u64,
}

impl TackyGen {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn program(&mut self, ast: ast::Program) -> Program {
        Program {
            function: self.function(ast.function_definition),
        }
    }

    fn function(&mut self, f: ast::FunctionDefinition) -> Function {
        Function {
            name: f.name,
            body: self.instructions(f.body),
        }
    }

    fn instructions(&mut self, stmt: ast::Statement) -> Vec<Instruction> {
        match stmt {
            ast::Statement::Return(expr) => {
                let mut instructions = vec![];
                let src = self.expression(expr, &mut instructions);
                let i = Instruction::Return(src);
                let mut is = Vec::with_capacity(instructions.len() + 1);
                is.append(&mut instructions);
                is.push(i);
                is
            }
        }
    }

    fn expression(&mut self, expr: ast::Expression, instructions: &mut Vec<Instruction>) -> Value {
        match expr {
            ast::Expression::Constant(n) => Value::Constant(n),
            ast::Expression::Unary(op, exp) => {
                let src = self.expression(*exp, instructions);
                let dst = self.make_temporary();
                let instruction = Instruction::Unary {
                    operator: op.into(),
                    src,
                    dst: dst.clone(),
                };
                instructions.push(instruction);
                dst
            }
            ast::Expression::Binary(op, left, right) => {
                let left = self.expression(*left, instructions);
                let right = self.expression(*right, instructions);
                let dst = self.make_temporary();
                let instruction = Instruction::Binary {
                    operator: op.into(),
                    left,
                    right,
                    dst: dst.clone(),
                };
                instructions.push(instruction);
                dst
            }
        }
    }

    fn make_temporary(&mut self) -> Value {
        let c = self.counter;
        self.counter += 1;
        Value::Var(format!("__tmp.{c}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tacky_constant() {
        let mut t = TackyGen::new();
        let stmt = ast::Statement::Return(ast::Expression::Constant(3));
        let i = t.instructions(stmt);
        assert_eq!(i, vec![Instruction::Return(Value::Constant(3))])
    }

    #[test]
    fn tacky_single_unary() {
        let mut t = TackyGen::new();
        let stmt = ast::Statement::Return(ast::Expression::Unary(
            ast::UnaryOperation::Complement,
            Box::new(ast::Expression::Constant(2)),
        ));
        let i = t.instructions(stmt);
        assert_eq!(
            i,
            vec![
                Instruction::Unary {
                    operator: UnaryOperator::Complement,
                    src: Value::Constant(2),
                    dst: Value::Var("__tmp.0".into())
                },
                Instruction::Return(Value::Var("__tmp.0".into()))
            ]
        )
    }

    #[test]
    fn tacky_nested_unary() {
        let mut t = TackyGen::new();
        let stmt = ast::Statement::Return(ast::Expression::Unary(
            ast::UnaryOperation::Negate,
            Box::new(ast::Expression::Unary(
                ast::UnaryOperation::Complement,
                Box::new(ast::Expression::Unary(
                    ast::UnaryOperation::Negate,
                    Box::new(ast::Expression::Constant(8)),
                )),
            )),
        ));
        let i = t.instructions(stmt);
        assert_eq!(
            i,
            vec![
                Instruction::Unary {
                    operator: UnaryOperator::Negate,
                    src: Value::Constant(8),
                    dst: Value::Var("__tmp.0".into())
                },
                Instruction::Unary {
                    operator: UnaryOperator::Complement,
                    src: Value::Var("__tmp.0".into()),
                    dst: Value::Var("__tmp.1".into())
                },
                Instruction::Unary {
                    operator: UnaryOperator::Negate,
                    src: Value::Var("__tmp.1".into()),
                    dst: Value::Var("__tmp.2".into())
                },
                Instruction::Return(Value::Var("__tmp.2".into()))
            ]
        )
    }

    #[test]
    fn tacky_single_binary() {
        let mut t = TackyGen::new();
        let stmt = ast::Statement::Return(ast::Expression::Binary(
            ast::BinaryOperation::Subtract,
            Box::new(ast::Expression::Constant(1)),
            Box::new(ast::Expression::Constant(2)),
        ));
        let i = t.instructions(stmt);
        assert_eq!(
            i,
            vec![
                Instruction::Binary {
                    operator: BinaryOperator::Subtract,
                    left: Value::Constant(1),
                    right: Value::Constant(2),
                    dst: Value::Var("__tmp.0".into())
                },
                Instruction::Return(Value::Var("__tmp.0".into()))
            ]
        )
    }

    #[test]
    fn tacky_nested_binary() {
        let mut t = TackyGen::new();
        let stmt = ast::Statement::Return(ast::Expression::Binary(
            ast::BinaryOperation::Subtract,
            Box::new(ast::Expression::Binary(
                ast::BinaryOperation::Subtract,
                Box::new(ast::Expression::Constant(1)),
                Box::new(ast::Expression::Constant(2)),
            )),
            Box::new(ast::Expression::Constant(3)),
        ));
        let i = t.instructions(stmt);
        assert_eq!(
            i,
            vec![
                Instruction::Binary {
                    operator: BinaryOperator::Subtract,
                    left: Value::Constant(1),
                    right: Value::Constant(2),
                    dst: Value::Var("__tmp.0".into())
                },
                Instruction::Binary {
                    operator: BinaryOperator::Subtract,
                    left: Value::Var("__tmp.0".into()),
                    right: Value::Constant(3),
                    dst: Value::Var("__tmp.1".into())
                },
                Instruction::Return(Value::Var("__tmp.1".into()))
            ]
        )
    }

    #[test]
    fn tacky_nested_precedence_binary() {
        let mut t = TackyGen::new();
        let stmt = ast::Statement::Return(ast::Expression::Binary(
            ast::BinaryOperation::Subtract,
            Box::new(ast::Expression::Binary(
                ast::BinaryOperation::Multiply,
                Box::new(ast::Expression::Constant(2)),
                Box::new(ast::Expression::Constant(3)),
            )),
            Box::new(ast::Expression::Constant(1)),
        ));
        let i = t.instructions(stmt);
        assert_eq!(
            i,
            vec![
                Instruction::Binary {
                    operator: BinaryOperator::Multiply,
                    left: Value::Constant(2),
                    right: Value::Constant(3),
                    dst: Value::Var("__tmp.0".into())
                },
                Instruction::Binary {
                    operator: BinaryOperator::Subtract,
                    left: Value::Var("__tmp.0".into()),
                    right: Value::Constant(1),
                    dst: Value::Var("__tmp.1".into())
                },
                Instruction::Return(Value::Var("__tmp.1".into()))
            ]
        )
    }
}

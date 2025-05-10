use std::{collections::HashMap, fmt::Display};

use crate::tacky;

pub struct Program {
    pub function: Function,
}

pub type Identifier = String;
pub struct Function {
    pub name: Identifier,
    pub instructions: Vec<Instruction>,
}

pub enum Instruction {
    Mov {
        src: Operand,
        dst: Operand,
    },
    Unary {
        operator: UnaryOperator,
        operand: Operand,
    },
    AllocateStack(u32),
    Ret,
}

pub enum UnaryOperator {
    Neg,
    Not,
}

impl From<tacky::UnaryOperator> for UnaryOperator {
    fn from(value: tacky::UnaryOperator) -> Self {
        match value {
            tacky::UnaryOperator::Complement => Self::Not,
            tacky::UnaryOperator::Negate => Self::Neg,
        }
    }
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            UnaryOperator::Neg => "negl",
            UnaryOperator::Not => "notl",
        };
        write!(f, "{out}")
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Imm(i32),
    Register(Register),
    Pseudo(Identifier),
    Stack(u32),
}

impl From<tacky::Value> for Operand {
    fn from(value: tacky::Value) -> Self {
        match value {
            tacky::Value::Constant(n) => Operand::Imm(n),
            tacky::Value::Var(s) => Operand::Pseudo(s),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Operand::Imm(n) => format!("${n}"),
            Operand::Register(r) => format!("{r}"),
            Operand::Pseudo(i) => format!("{i}"),
            Operand::Stack(i) => format!("-{i}(%rbp)"),
        };
        write!(f, "{out}")
    }
}

#[derive(Debug, Clone)]
pub enum Register {
    AX,
    R10,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Register::AX => "%eax",
            Register::R10 => "%r10d",
        };

        write!(f, "{out}")
    }
}

pub fn assembly(program: tacky::Program) -> Program {
    let p = Program {
        function: function_definition(program.function),
    };

    let (p, offset) = replace_pseudo_registers(p);
    fixing_up(p, offset)
}

fn function_definition(function: tacky::Function) -> Function {
    Function {
        name: function.name,
        instructions: instructions(function.body),
    }
}

fn instructions(is: Vec<tacky::Instruction>) -> Vec<Instruction> {
    is.into_iter()
        .flat_map(|i| match i {
            tacky::Instruction::Return(v) => vec![
                Instruction::Mov {
                    src: v.into(),
                    dst: Operand::Register(Register::AX),
                },
                Instruction::Ret,
            ],
            tacky::Instruction::Unary { operator, src, dst } => {
                let dst: Operand = dst.into();
                vec![
                    Instruction::Mov {
                        src: src.into(),
                        dst: dst.clone(),
                    },
                    Instruction::Unary {
                        operator: operator.into(),
                        operand: dst,
                    },
                ]
            }
            tacky::Instruction::Binary {
                operator,
                left,
                right,
                dst,
            } => {
                todo!()
            }
        })
        .collect()
}

fn replace_pseudo_registers(mut program: Program) -> (Program, u32) {
    let mut map: HashMap<String, u32> = HashMap::new();
    let mut offset = 0;
    program.function.instructions = program
        .function
        .instructions
        .into_iter()
        .map(|i| match i {
            Instruction::Mov { src, dst } => {
                let (src, of) = stack_offset(src, &mut map, offset);
                let (dst, of) = stack_offset(dst, &mut map, of);
                offset = of;
                Instruction::Mov { src, dst }
            }
            Instruction::Unary { operator, operand } => {
                let (operand, of) = stack_offset(operand, &mut map, offset);
                offset = of;
                Instruction::Unary { operator, operand }
            }
            i @ (Instruction::AllocateStack(_) | Instruction::Ret) => i,
        })
        .collect::<Vec<_>>();
    (program, offset)
}

fn stack_offset(op: Operand, map: &mut HashMap<String, u32>, offset: u32) -> (Operand, u32) {
    if let Operand::Pseudo(i) = op {
        let e = map.entry(i);
        let offset = *e.or_insert(offset + 4);
        return (Operand::Stack(offset), offset);
    }
    (op, offset)
}

fn fixing_up(mut program: Program, stack_size: u32) -> Program {
    program
        .function
        .instructions
        .insert(0, Instruction::AllocateStack(stack_size));
    program.function.instructions = program
        .function
        .instructions
        .into_iter()
        .flat_map(|i| match i {
            Instruction::Mov {
                src: src @ Operand::Stack(_),
                dst: dst @ Operand::Stack(_),
            } => {
                vec![
                    Instruction::Mov {
                        src,
                        dst: Operand::Register(Register::R10),
                    },
                    Instruction::Mov {
                        src: Operand::Register(Register::R10),
                        dst,
                    },
                ]
            }
            i @ _ => vec![i],
        })
        .collect();
    program
}

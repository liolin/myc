use crate::assembly;

pub fn codegen(assembly: assembly::Program) -> String {
    program(assembly)
}

fn program(program: assembly::Program) -> String {
    let function = function_definition(program.function_definition);
    format!("{function}\n\n.section .note.GNU-stack,\"\",@progbits")
}

fn function_definition(function: assembly::FunctionDefinition) -> String {
    let is = function
        .instructions
        .into_iter()
        .map(|i| instruction(i))
        .collect::<Vec<_>>()
        .join("\n\t");

    format!("\t.global {}\n{}:\n\t{}", function.name, function.name, is)
}

fn instruction(instruction: assembly::Instruction) -> String {
    match instruction {
        assembly::Instruction::Mov(m) => mov(m),
        assembly::Instruction::Ret => "ret".into(),
    }
}

fn mov(mov: assembly::Mov) -> String {
    format!("movl {}, {}", operand(mov.src), operand(mov.dst))
}

fn operand(operand: assembly::Operand) -> String {
    match operand {
        assembly::Operand::Imm(i) => format!("${i}"),
        assembly::Operand::Register => format!("%eax"),
    }
}

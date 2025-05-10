use crate::assembly;

pub fn codegen(assembly: assembly::Program) -> String {
    program(assembly)
}

fn program(program: assembly::Program) -> String {
    let function = function_definition(program.function);
    format!("{function}\n\n\t.section .note.GNU-stack,\"\",@progbits")
}

fn function_definition(function: assembly::Function) -> String {
    let is = function
        .instructions
        .into_iter()
        .map(|i| instruction(i))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "\t.global {}\n{}:\n\tpushq\t%rbp\n\tmovq\t%rsp, %rbp\n{}",
        function.name, function.name, is
    )
}

fn instruction(instruction: assembly::Instruction) -> String {
    match instruction {
        assembly::Instruction::Mov { src, dst } => {
            format!("\tmovl\t{}, {}", operand(src), operand(dst))
        }
        assembly::Instruction::Unary { operator, operand } => {
            format!("\t{}\t{}", operator, operand)
        }
        assembly::Instruction::AllocateStack(i) => {
            format!("\tsubq\t${i}, %rsp")
        }
        assembly::Instruction::Ret => "\tmovq\t%rbp, %rsp\n\tpopq\t%rbp\n\tret".into(),
    }
}

fn operand(operand: assembly::Operand) -> String {
    if let assembly::Operand::Pseudo(i) = operand {
        panic!("Found pseudo register, should be replaced: {i}");
    }
    operand.to_string()
}

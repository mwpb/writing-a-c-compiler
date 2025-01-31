use crate::ast::{self, Expression, Identifier, Statement};
enum Operand {
    Imm(i32),
    Register,
}

struct Move {
    src: Operand,
    dst: Operand,
}

enum Instruction {
    Ret,
    Mov(Move),
}

struct Function {
    name: String,
    instructions: Vec<Instruction>,
}

pub struct Programme {
    function_definition: Function,
}

fn generate_operand(expression: ast::Expression) -> Operand {
    match expression {
        Expression::Constant(i) => Operand::Imm(i),
        Expression::Unary(_) => todo!(),
    }
}

fn generate_instructions(statement: ast::Statement) -> Vec<Instruction> {
    match statement {
        Statement::Return(exp) => vec![
            Instruction::Mov(Move {
                src: generate_operand(exp),
                dst: Operand::Register,
            }),
            Instruction::Ret,
        ],
    }
}

fn generate_function_name(identifier: ast::Identifier) -> String {
    match identifier {
        Identifier::Var(val) => val,
    }
}

fn generate_function(function: ast::Function) -> Function {
    Function {
        name: generate_function_name(function.name),
        instructions: generate_instructions(function.body),
    }
}

pub fn generate_programme(programme: ast::Programme) -> Programme {
    Programme {
        function_definition: generate_function(programme.function),
    }
}

fn print_operand(operand: Operand) -> String {
    match operand {
        Operand::Imm(i) => format!("#{}", i),
        Operand::Register => r"w0".to_string(),
    }
}

fn print_instruction(instruction: Instruction) -> String {
    match instruction {
        Instruction::Ret => "ret".to_string(),
        Instruction::Mov(mov) => format!(
            "mov    {}, {}",
            print_operand(mov.dst),
            print_operand(mov.src),
        ),
    }
}

fn print_function_definition(function: Function) -> String {
    let name = function.name;
    let mut output = format!("    .globl _{name}\n_{name}:\n").to_string();
    for instruction in function.instructions {
        let instruction_text = print_instruction(instruction);
        output += &format!("    {}\n", instruction_text)
    }
    output
}
pub fn print_programme(programme: Programme) -> String {
    print_function_definition(programme.function_definition)
}

use std::{error::Error, fmt::Display};

use super::{instruction::Instruction, parser::AST};

#[derive(Default)]
struct Generator {
    pc: usize,
    insts: Vec<Instruction>,
}

impl Generator {
    fn gen_code(&self, ast: &AST) -> Result<(), CodeGenError> {
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum CodeGenError {
    PCOverFlow,
    FailStar,
    FailOr,
    FailQuestion,
}

impl Display for CodeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CodeGenError: {:?}", self)
    }
}

pub(crate) fn get_code(ast: &AST) -> Result<Vec<Instruction>, CodeGenError> {
    let mut generator = Generator::default();
    generator.gen_code(ast)?;
    Ok(generator.insts)
}

impl Error for CodeGenError {}

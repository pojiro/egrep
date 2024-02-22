use std::{error::Error, fmt::Display};

use super::instruction::Instruction;

#[derive(Debug)]
pub(crate) enum EvalError {
    PCOverFlow,
    SPOverFlow,
    InvalidPC,
    InvalidContext,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EvalError: {:?}", self)
    }
}

pub(crate) fn eval(inst: &[Instruction], line: &[char]) -> Result<bool, EvalError> {
    Ok(true)
}

impl Error for EvalError {}

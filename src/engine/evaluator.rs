use std::{error::Error, fmt::Display};

use crate::helper::safe_add;

use super::instruction::Instruction;

#[derive(Debug)]
pub(crate) enum EvalError {
    PCOverFlow,
    SPOverFlow,
    InvalidPC,
    // InvalidContext,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EvalError: {:?}", self)
    }
}

pub(crate) fn eval(insts: &[Instruction], line: &[char]) -> Result<bool, EvalError> {
    eval_depth(insts, line, 0, 0)
}

impl Error for EvalError {}

fn eval_depth(
    insts: &[Instruction],
    line: &[char],
    mut pc: usize,
    mut sp: usize,
) -> Result<bool, EvalError> {
    loop {
        let next_inst = if let Some(inst) = insts.get(pc) {
            inst
        } else {
            return Err(EvalError::InvalidPC);
        };

        match next_inst {
            Instruction::Char(c) => {
                if let Some(sp_c) = line.get(sp) {
                    if c == sp_c {
                        safe_add(&mut pc, &1, || EvalError::PCOverFlow)?;
                        safe_add(&mut sp, &1, || EvalError::SPOverFlow)?
                    } else {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            Instruction::Match => return Ok(true),
            Instruction::Jump(addr) => pc = *addr,
            Instruction::Split(addr1, addr2) => {
                if eval_depth(insts, line, *addr1, sp)? || eval_depth(insts, line, *addr2, sp)? {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }
        }
    }
}

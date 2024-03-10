use crate::helper::safe_add;
use std::{error::Error, fmt::Display};

use super::{instruction::Instruction, parser::AST};

#[derive(Default)]
struct Generator {
    pc: usize,
    insts: Vec<Instruction>,
}

impl Generator {
    fn gen_code(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        self.gen_expr(ast)?;
        self.inc_pc()?;
        self.insts.push(Instruction::Match);
        Ok(())
    }

    fn gen_expr(&mut self, ast: &AST) -> Result<(), CodeGenError> {
        match ast {
            AST::Char(c) => self.gen_char(*c)?,
            AST::Plus(e) => self.gen_plus(e)?,
            AST::Star(e) => self.gen_star(e)?,
            AST::Question(e) => self.gen_question(e)?,
            AST::Or(e1, e2) => self.gen_or(e1, e2)?,
            AST::Seq(v) => self.gen_seq(v)?,
        }
        Ok(())
    }

    fn inc_pc(&mut self) -> Result<(), CodeGenError> {
        safe_add(&mut self.pc, &1, || CodeGenError::PCOverFlow)
    }

    fn gen_char(&mut self, c: char) -> Result<(), CodeGenError> {
        self.insts.push(Instruction::Char(c));
        self.inc_pc()?;
        Ok(())
    }

    fn gen_plus(&mut self, e: &AST) -> Result<(), CodeGenError> {
        let l1 = self.pc;
        self.gen_expr(e)?;

        let l2 = self.pc;
        let split = Instruction::Split(l1, 0);
        self.insts.push(split);
        self.inc_pc()?;

        if let Some(Instruction::Split(_, l3)) = self.insts.get_mut(l2) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailPlus);
        }

        Ok(())
    }

    fn gen_star(&mut self, e: &AST) -> Result<(), CodeGenError> {
        let l1 = self.pc;
        let split = Instruction::Split(0, 0);
        self.insts.push(split);
        self.inc_pc()?;

        if let Some(Instruction::Split(l2, _)) = self.insts.get_mut(l1) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailStar);
        }

        self.gen_expr(e)?;

        let jump = Instruction::Jump(l1);
        self.insts.push(jump);
        self.inc_pc()?;

        if let Some(Instruction::Split(_, l3)) = self.insts.get_mut(l1) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailStar);
        }

        Ok(())
    }

    fn gen_question(&mut self, e: &AST) -> Result<(), CodeGenError> {
        let split_addr = self.pc;
        let split = Instruction::Split(0, 0);
        self.insts.push(split);
        self.inc_pc()?;

        if let Some(Instruction::Split(l1, _)) = self.insts.get_mut(split_addr) {
            *l1 = self.pc;
        } else {
            return Err(CodeGenError::FailQuestion);
        }

        self.gen_expr(e)?;

        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailQuestion);
        }

        Ok(())
    }

    fn gen_or(&mut self, e1: &AST, e2: &AST) -> Result<(), CodeGenError> {
        let split_addr = self.pc;
        self.inc_pc()?;
        let split = Instruction::Split(self.pc, 0);
        self.insts.push(split);

        self.gen_expr(e1)?;

        let jmp_addr = self.pc;
        self.insts.push(Instruction::Jump(0));

        self.inc_pc()?;
        if let Some(Instruction::Split(_, l2)) = self.insts.get_mut(split_addr) {
            *l2 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        self.gen_expr(e2)?;

        if let Some(Instruction::Jump(l3)) = self.insts.get_mut(jmp_addr) {
            *l3 = self.pc;
        } else {
            return Err(CodeGenError::FailOr);
        }

        Ok(())
    }

    fn gen_seq(&mut self, v: &Vec<AST>) -> Result<(), CodeGenError> {
        for e in v {
            self.gen_expr(e)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum CodeGenError {
    PCOverFlow,
    FailOr,
    FailPlus,
    FailStar,
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

#[cfg(test)]
mod tests {
    use crate::engine::codegen::Generator;
    use crate::engine::instruction::Instruction;
    use crate::engine::parser::AST;

    #[test]
    fn test_gen_plus() {
        let mut generator = Generator::default();
        let _ = generator.gen_code(&AST::Plus(Box::new(AST::Char('a'))));
        assert_eq!(
            generator.insts,
            vec![
                Instruction::Char('a'),
                Instruction::Split(0, 2),
                Instruction::Match
            ]
        )
    }

    #[test]
    fn test_gen_star() {
        let mut generator = Generator::default();
        let _ = generator.gen_code(&AST::Star(Box::new(AST::Char('a'))));
        assert_eq!(
            generator.insts,
            vec![
                Instruction::Split(1, 3),
                Instruction::Char('a'),
                Instruction::Jump(0),
                Instruction::Match
            ]
        )
    }

    #[test]
    fn test_gen_question() {
        let mut generator = Generator::default();
        let _ = generator.gen_code(&AST::Question(Box::new(AST::Char('a'))));
        assert_eq!(
            generator.insts,
            vec![
                Instruction::Split(1, 2),
                Instruction::Char('a'),
                Instruction::Match
            ]
        )
    }
}

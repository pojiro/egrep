use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub(crate) enum ParseError {
    Empty,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub(crate) enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>),
    Seq(Box<AST>),
}

pub(crate) fn parse(expr: &str) -> Result<AST, ParseError> {
    Err(ParseError::Empty)
}

impl Error for ParseError {}

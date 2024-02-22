mod codegen;
mod evaluator;
mod instruction;
mod parser;

use std::error::Error;

pub(crate) fn do_matching(expr: &str, line: &str) -> Result<bool, Box<dyn Error>> {
    let ast = parser::parse(expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    Ok(evaluator::eval(&code, &line)?)
}

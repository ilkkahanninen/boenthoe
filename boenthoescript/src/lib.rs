use std::collections::HashMap;

mod ast;
mod compiler;
mod envelope;
mod parser;
mod vector;

pub use crate::compiler::EnvelopeFn;
pub use crate::vector::Vector;

pub fn build(source: &str) -> Result<HashMap<String, EnvelopeFn>, String> {
    match parser::parse(source) {
        Ok(ast) => {
            // println!("AST: {:?}", ast);
            compiler::build(ast).or_else(|err| Err(format!("Could not compile: {:?}", err)))
        }
        Err(error) => Err(format!("Could not parse: {:?}", error)),
    }
}

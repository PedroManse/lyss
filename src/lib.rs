use std::path::PathBuf;

use self::parser::FnName;
pub mod tokenizer;
pub mod parser;

#[derive(Debug)]
pub enum Value {
    Str(String),
    Num(f64),
    List(Vec<Value>),
    Ident(FnName),
}

#[derive(Debug)]
pub enum LyssCompError {
    CantStopToken{
        line: usize,
        file: PathBuf,
        tokenizer_state: tokenizer::State,
    }
}

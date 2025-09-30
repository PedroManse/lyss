use std::path::PathBuf;

use self::parser::{Code, ExprCont, FnName};
use self::tokenizer::Token;
pub mod tokenizer;
pub mod display;
pub mod parser;
pub mod runtime;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Num(f64),
    List(Vec<Value>),
    Ident(FnName),
    Code(Code),
}

#[derive(Debug)]
pub enum LyssCompError {
    CodeWithoutRootAtom{
        first_token: Option<Token>,
    },
    ParseFloat(std::num::ParseFloatError),
    CantStopToken{
        line: usize,
        file: PathBuf,
        tokenizer_state: tokenizer::State,
    }
}

#[derive(Debug)]
pub enum LyssRuntimeError {
    EntryNotFound{path: Vec<String>},
    EntryWasLeaf{path: Vec<String>},
    EntryWasBranch{path: Vec<String>},
}


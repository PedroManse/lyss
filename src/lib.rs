use std::path::PathBuf;

use self::parser::{Argument, Code, ExprCont, FnName};
use self::tokenizer::Token;
pub mod display;
pub mod parser;
pub mod runtime;
pub mod tokenizer;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Str(String),
    Num(f64),
    List(Vec<Value>),
    //Ident(FnName),
    Code(Code),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Num(a), Value::Num(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::List(_), _) => false,
            (Value::Code(_), _) => false,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum LyssCompError {
    CodeWithoutRootAtom {
        first_token: Option<Token>,
    },
    ParseFloat(std::num::ParseFloatError),
    CantStopToken {
        line: usize,
        file: PathBuf,
        tokenizer_state: tokenizer::State,
    },
}

#[derive(Debug)]
pub enum LyssRuntimeError {
    EntryNotFound {
        path: Vec<String>,
    },
    EntryWasLeaf {
        path: Vec<String>,
    },
    EntryWasBranch {
        path: Vec<String>,
    },
    VarNotFound {
        name: String,
    },
    LiteralNotFound {
        expected: String,
        got: Argument,
    },
    UnexpectedArg {
        arg: Argument,
        expected: &'static str,
    },
    NeedsArg,
    TooManyArgs {
        got: Vec<Argument>,
        needs: usize,
    },
    TooFewArgs {
        got: Vec<Argument>,
        needs: usize,
    },
    UnmatchedArgCount {
        got: Vec<Argument>,
        could_usize: Vec<usize>,
    },
}

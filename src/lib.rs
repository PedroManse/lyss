use std::path::PathBuf;
pub mod tokenizer;
pub mod parser;

#[derive(Debug)]
pub enum LyssCompError {
    CantStopToken{
        line: usize,
        file: PathBuf,
        tokenizer_state: tokenizer::State,
    }
}

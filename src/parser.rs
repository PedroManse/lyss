use crate::tokenizer::Token;
use crate::LyssCompError;

pub struct Expr {
    line: usize,
    cont: ExprCont,
}

pub struct Atom {

}

pub enum ExprCont {
    Atom(Atom),
    Macro{name: String, content: String},
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expr>, LyssCompError> {
    todo!()
}

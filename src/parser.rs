#![allow(dead_code)]

use crate::tokenizer::{Token, TokenCont};
use crate::{LyssCompError, Value};

#[derive(Debug)]
pub struct Expr {
    line_span: std::ops::Range<usize>,
    cont: ExprCont,
}

#[derive(Debug)]
pub enum FnName {
    Single(String),
    Path(Vec<String>),
}

#[derive(Debug)]
pub struct Atom {
    line_span: std::ops::Range<usize>,
    fn_name: FnName,
    arguments: Vec<Argument>,
}

#[derive(Debug)]
pub enum Argument {
    Atom(Atom),
    Value(Value),
    Macro(MacroDef),
}

#[derive(Debug)]
pub enum ExprCont {
    Atom(Atom),
    Macro(MacroDef),
}

#[derive(Debug)]
pub struct MacroDef {
    name: String,
    content: String,
}

#[derive(Debug)]
pub enum State {
    OnAtom,
    OnArgs(FnName, Vec<Argument>),
}

pub fn parse_atom(
    start_line: usize,
    tokens: &mut impl Iterator<Item = Token>,
) -> Result<Atom, LyssCompError> {
    let mut state = State::OnAtom;
    while let Some(Token { line, content }) = tokens.next() {
        state = match (state, content) {
            (State::OnAtom, TokenCont::Ident(cnt)) => State::OnArgs(FnName::Single(cnt), vec![]),
            (State::OnAtom, TokenCont::Path(cnt)) => State::OnArgs(FnName::Path(cnt), vec![]),
            (State::OnArgs(fn_name, mut args), TokenCont::OParam) => {
                args.push(Argument::Atom(parse_atom(line, tokens)?));
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, mut args), TokenCont::String(cnt)) => {
                args.push(Argument::Value(Value::Str(cnt)));
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, mut args), TokenCont::Path(secs)) => {
                args.push(Argument::Value(Value::Ident(FnName::Path(secs))));
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, mut args), TokenCont::Ident(cnt)) => {
                args.push(Argument::Value(Value::Ident(FnName::Single(cnt))));
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, mut args), TokenCont::Macro { name, content, .. }) => {
                args.push(Argument::Macro(MacroDef { name, content }));
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, arguments), TokenCont::CParam) => {
                return Ok(Atom {
                    fn_name,
                    arguments,
                    line_span: start_line..line,
                });
            }

            (s, t) => panic!("#{line}: Can't advance {s:?} with {t:?}"),
        };
    }
    panic!()
}

pub fn parse(mut tokens: impl Iterator<Item = Token>) -> Result<Vec<Expr>, LyssCompError> {
    let mut exprs = vec![];
    while let Some(Token { line, content }) = tokens.next() {
        let expr = match content {
            TokenCont::OParam => {
                let atom = parse_atom(line, &mut tokens)?;
                let line_span = atom.line_span.clone();
                let cont = ExprCont::Atom(atom);
                Expr { cont, line_span }
            }
            TokenCont::Macro {
                name,
                content,
                line_span,
            } => {
                let atom = MacroDef { name, content };
                let cont = ExprCont::Macro(atom);
                Expr { cont, line_span }
            }
            t => panic!("Top level expression must be either macro or atom, got {t:?}"),
        };
        exprs.push(expr);
    }
    Ok(exprs)
}

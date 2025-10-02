#![allow(dead_code)]

use crate::tokenizer::{Token, TokenCont};
use crate::{LyssCompError, Value};

#[derive(Debug, Clone)]
pub struct Expr {
    pub line_span: std::ops::Range<usize>,
    pub cont: ExprCont,
}

#[derive(Debug, Clone)]
pub struct FnName(pub Vec<String>);

#[derive(Debug, Clone)]
pub struct Atom {
    pub line_span: std::ops::Range<usize>,
    pub fn_name: FnName,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub enum Argument {
    Var(String),
    Ident(FnName),
    Atom(Atom),
    Value(Value),
    Macro(MacroUse),
}

#[derive(Debug, Clone)]
pub struct Code {
    pub line_span: std::ops::Range<usize>,
    pub exprs: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum ExprCont {
    Atom(Atom),
    Macro(MacroUse),
    //Code(Code),
}

#[derive(Debug, Clone)]
pub struct MacroUse {
    pub name: String,
    pub content: String,
}

#[derive(Debug)]
pub enum AtomState {
    OnAtom,
    OnArgs(FnName, Vec<Argument>),
}

pub fn parse_atom(
    start_line: usize,
    tokens: &mut impl Iterator<Item = Token>,
) -> Result<Atom, LyssCompError> {
    use AtomState as State;
    let mut state = State::OnAtom;
    while let Some(Token { line, content }) = tokens.next() {
        state = match (state, content) {
            (State::OnAtom, TokenCont::Ident(cnt)) => State::OnArgs(FnName(vec![cnt]), vec![]),
            (State::OnAtom, TokenCont::Path(cnt)) => State::OnArgs(FnName(cnt), vec![]),
            (State::OnArgs(fn_name, mut args), TokenCont::OParam) => {
                args.push(Argument::Atom(parse_atom(line, tokens)?));
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, mut args), TokenCont::String(cnt)) => {
                args.push(Argument::Value(Value::Str(cnt)));
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, mut args), TokenCont::Digit(cnt)) => {
                let num = cnt.parse().map_err(LyssCompError::ParseFloat)?;
                args.push(Argument::Value(Value::Num(num)));
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, mut args), TokenCont::Path(mut secs)) => {
                if secs.len() == 2 && secs.first().map(String::as_str) == Some("$") {
                    args.push(Argument::Var(secs.swap_remove(1)));
                } else {
                    args.push(Argument::Ident(FnName(secs)));
                }
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, mut args), TokenCont::Ident(cnt)) => {
                args.push(Argument::Ident(FnName(vec![cnt])));
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, mut args), TokenCont::Macro { name, content, .. }) => {
                args.push(Argument::Macro(MacroUse { name, content }));
                State::OnArgs(fn_name, args)
            }
            (State::OnArgs(fn_name, mut args), TokenCont::SingleQuote) => {
                let (exprs, last_line) = parse_code(tokens)?;
                args.push(Argument::Value(Value::Code(Code {
                    exprs,
                    line_span: line..last_line,
                })));
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

pub enum CodeState {
    BeforeCode, // needs (
    OnCode,
    OnCodeEnd, // read ', needs )
}

pub fn parse_code(
    tokens: &mut impl Iterator<Item = Token>,
) -> Result<(Vec<Expr>, usize), LyssCompError> {
    match tokens.next() {
        Some(Token {
            content: TokenCont::OParam,
            ..
        }) => {
            let mut exprs = vec![];
            loop {
                let token = tokens.next().unwrap();
                if let TokenCont::CParam = token.content {
                    break;
                }
                let expr = parse_once(token, tokens)?;
                exprs.push(expr);
            }
            let Token { line, content } = tokens.next().unwrap();
            if let TokenCont::SingleQuote = content {
                Ok((exprs, line))
            } else {
                panic!("Didn't end with close param {content:?}");
            }
        }
        tkn => Err(LyssCompError::CodeWithoutRootAtom { first_token: tkn }),
    }
}

pub fn parse_once(
    Token { line, content }: Token,
    tokens: &mut impl Iterator<Item = Token>,
) -> Result<Expr, LyssCompError> {
    Ok(match content {
        TokenCont::OParam => {
            let atom = parse_atom(line, tokens)?;
            let line_span = atom.line_span.clone();
            let cont = ExprCont::Atom(atom);
            Expr { line_span, cont }
        }
        //TokenCont::SingleQuote => {
        //    let (exprs, end_line) = parse_code(tokens)?;
        //    Expr {
        //        line_span: line..end_line,
        //        cont: ExprCont::Code(Code {
        //            exprs,
        //            line_span: line..end_line,
        //        }),
        //    }
        //}
        TokenCont::Macro {
            name,
            content,
            line_span,
        } => {
            let atom = MacroUse { name, content };
            let cont = ExprCont::Macro(atom);
            Expr { line_span, cont }
        }
        t => panic!("Top level expression must be either macro or atom, got {t:?}"),
    })
}

pub fn parse(tokens: &mut impl Iterator<Item = Token>) -> Result<Vec<Expr>, LyssCompError> {
    let mut exprs = vec![];
    while let Some(token) = tokens.next() {
        let expr = parse_once(token, tokens)?;
        exprs.push(expr);
    }
    Ok(exprs)
}

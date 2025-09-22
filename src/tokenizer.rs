use std::path::Path;

use crate::LyssCompError;

#[derive(Debug)]
pub enum TokenCont {
    OParam,
    CParam,
    SingleQuote,
    MacroQuote,
    Ident(String),
    String(String),
    Digit(String),
    Dot,
    Macro { name: String, content: String },
}

#[derive(Debug)]
pub struct Token {
    line: usize,
    content: TokenCont,
}

macro_rules! char_group {
    (space) => {
        ' ' | '\n' | '\t'
    };
    (alphabet) => {
        'A'..='Z' | 'a'..='z'
    };
    (digit) => {
        '0'..='9'
    };
    (ident_start) => {
        '$' | '=' | '<' | '>' | '_' | '-' | '+' | '/' | '*' | char_group!(alphabet)
    };
    (ident) => {
        char_group!(ident_start) | '.' | '!'
    };
}

#[derive(Debug)]
pub enum State {
    Nothing,
    Comment,
    String(String),
    StringSlash(String),
    Ident(String),
    Digit(String),
    DigitDot(String),
    MacroWaitAtom,
    MacroWaitContent(String),
    Macro {
        name: String,
        content: String,
        parem_depth: usize,
    },
}

impl State {
    fn into_token(self, line: usize, file: &Path) -> Result<Option<TokenCont>, LyssCompError> {
        Ok(Some(match self {
            State::Nothing => return Ok(None),
            State::String(cnt) => TokenCont::String(cnt),
            State::Ident(cnt) => TokenCont::Ident(cnt),
            State::Digit(cnt) => TokenCont::Digit(cnt),
            State::DigitDot(cnt) => TokenCont::Digit(cnt),
            State::Comment
            | State::StringSlash(..)
            | State::Macro { .. }
            | State::MacroWaitContent(..)
            | State::MacroWaitAtom => {
                return Err(LyssCompError::CantStopToken {
                    line,
                    file: file.to_path_buf(),
                    tokenizer_state: self,
                });
            }
        }))
    }
}

pub fn tokenize(content: &str, file_name: &Path) -> Result<Vec<Token>, LyssCompError> {
    let mut state = State::Nothing;
    let mut tokens = Vec::new();
    let mut line = 1;
    macro_rules! token {
        ($tkn:expr) => {
            Token {
                line,
                content: $tkn,
            }
        };
    }

    for c in content.chars() {
        state = match (state, c) {
            // Comment
            (State::Comment, '\n') => State::Nothing,
            (State::Comment, _) => State::Comment,
            (State::Nothing, '#') => State::Comment,

            // String
            (State::Nothing, '"') => State::String(String::new()),
            (State::String(cnt), '\\') => State::StringSlash(cnt),
            (State::StringSlash(mut cnt), c @ char_group!(space)) => {
                cnt.push(c);
                State::String(cnt)
            }
            (State::String(cnt), '"') => {
                tokens.push(token!(TokenCont::String(cnt)));
                State::Nothing
            }
            (State::String(mut cnt), c) => {
                cnt.push(c);
                State::String(cnt)
            }

            // Digit
            (State::Nothing, d @ char_group!(digit)) => State::Digit(String::from(d)),
            (State::Nothing, c @ '.') => State::DigitDot(String::from(c)),
            (State::Digit(mut cnt) | State::DigitDot(mut cnt), d @ char_group!(digit)) => {
                cnt.push(d);
                State::Digit(cnt)
            }
            (State::Digit(mut cnt), '.') => {
                cnt.push('.');
                State::DigitDot(cnt)
            }
            (State::Digit(cnt) | State::DigitDot(cnt), char_group!(space)) => {
                if cnt == "." {
                    tokens.push(token!(TokenCont::Dot));
                } else {
                    tokens.push(token!(TokenCont::Digit(cnt)));
                }
                State::Nothing
            }

            // Ident
            (State::Nothing, c @ char_group!(ident_start)) => State::Ident(String::from(c)),
            (State::Ident(mut cnt), c @ char_group!(ident)) => {
                cnt.push(c);
                State::Ident(cnt)
            }
            (State::Ident(cnt), char_group!(space)) => {
                tokens.push(token!(TokenCont::Ident(cnt)));
                State::Nothing
            }

            // Macro content
            (State::Nothing, '!') => State::MacroWaitAtom,
            (State::MacroWaitAtom, '(') => State::MacroWaitContent(String::new()),
            (State::MacroWaitContent(mut name), char_group!(ident)) => {
                name.push(c);
                State::MacroWaitContent(name)
            }
            (State::MacroWaitContent(name), ' ') => State::Macro {
                name,
                content: String::new(),
                parem_depth: 0,
            },
            (
                State::Macro {
                    name,
                    mut content,
                    parem_depth,
                },
                '(',
            ) => {
                content.push('(');
                State::Macro {
                    name,
                    content,
                    parem_depth: parem_depth + 1,
                }
            }
            (
                State::Macro {
                    name,
                    content,
                    parem_depth: 0,
                },
                ')',
            ) => {
                tokens.push(token!(TokenCont::Macro { name, content }));
                State::Nothing
            }
            (
                State::Macro {
                    name,
                    mut content,
                    parem_depth,
                },
                ')',
            ) => {
                content.push(')');
                State::Macro {
                    name,
                    content,
                    parem_depth: parem_depth - 1,
                }
            }
            (
                State::Macro {
                    name,
                    mut content,
                    parem_depth,
                },
                c,
            ) => {
                content.push(c);
                State::Macro {
                    name,
                    content,
                    parem_depth,
                }
            }

            // Params
            (s, '(') => {
                if let Some(tkn) = s.into_token(line, file_name)? {
                    tokens.push(token!(tkn));
                }
                tokens.push(token!(TokenCont::CParam));
                State::Nothing
            }
            (s, ')') => {
                if let Some(tkn) = s.into_token(line, file_name)? {
                    tokens.push(token!(tkn));
                }
                tokens.push(token!(TokenCont::CParam));
                State::Nothing
            }

            // Single/Macro quotes
            (State::Nothing, '\'') => {
                tokens.push(token!(TokenCont::SingleQuote));
                State::Nothing
            }
            (State::Nothing, '`') => {
                tokens.push(token!(TokenCont::MacroQuote));
                State::Nothing
            }

            (s, char_group!(space)) => s,

            (s, c) => todo!("{file_name:?}#{line}: {s:?} -> '{c}'"),
        };
        if c == '\n' {
            line += 1;
        }
    }
    Ok(tokens)
}

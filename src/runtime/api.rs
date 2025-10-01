use super::*;
pub struct Api;

impl Context<'_> {
    pub fn eval_argument(&mut self, argument: &Argument) -> Result<Value, LyssRuntimeError> {
        Ok(match argument {
            Argument::Var(name) => self
                .get_var(name)
                .ok_or(LyssRuntimeError::VarNotFound {
                    name: name.to_owned(),
                })?
                .clone(),
            Argument::Atom(atom) => self.execute_atom(atom)?,
            Argument::Value(v) => v.clone(),
            Argument::Macro(m) => todo!("macro argument {m}"),
            Argument::Ident(path) => todo!(),
        })
    }
}

impl Api {
    pub fn expect_this_text(argument: &Argument, text: &str) -> Result<(), LyssRuntimeError> {
        if let Argument::Value(Value::Ident(path)) = argument
            && path.0.len() == 1
            && path.0.first().map(String::as_str) == Some(text)
        {
            Ok(())
        } else {
            Err(LyssRuntimeError::LiteralNotFound {
                expected: text.to_string(),
                got: argument.clone(),
            })
        }
    }
    #[must_use]
    pub fn expect_ident(argument: &Argument) -> Option<&str> {
        match argument {
            Argument::Var(txt) => Some(txt),
            _ => None,
        }
    }
    #[must_use]
    pub fn expect_atom(argument: &Argument) -> Option<&Atom> {
        match argument {
            Argument::Atom(atom) => Some(atom),
            _ => None,
        }
    }
    #[must_use]
    pub fn expect_literal(argument: &Argument) -> Option<&Value> {
        match argument {
            Argument::Value(v) => Some(v),
            _ => None,
        }
    }
    pub fn needs_nth_arg(args: &[Argument], index: usize) -> Result<&Argument, LyssRuntimeError> {
        args.get(index).ok_or(LyssRuntimeError::NeedsArg)
    }
    pub fn assert_args_count(args: &[Argument], count: usize) -> Result<(), LyssRuntimeError> {
        match args.len().cmp(&count) {
            std::cmp::Ordering::Less => Err(LyssRuntimeError::TooFewArgs {
                got: args.to_vec(),
                needs: count,
            }),
            std::cmp::Ordering::Greater => Err(LyssRuntimeError::TooManyArgs {
                got: args.to_vec(),
                needs: count,
            }),
            std::cmp::Ordering::Equal => Ok(()),
        }
    }
    pub fn assert_args_valid_counts<const N: usize>(
        args: &[Argument],
        valid_counts: [usize; N],
    ) -> Result<(), LyssRuntimeError> {
        for vc in valid_counts {
            if args.len() == vc {
                return Ok(());
            }
        }
        Err(LyssRuntimeError::UnmatchedArgCount {
            got: args.to_vec(),
            could_usize: valid_counts.to_vec(),
        })
    }
}

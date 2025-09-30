use std::fmt::Display;

impl Display for crate::Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::Value::Str(cnt) => write!(f, "\"{}\"", cnt),
            crate::Value::Num(cnt) => write!(f, "{}", cnt),
            crate::Value::List(cnt) => {
                write!(f, "[")?;
                for v in cnt {
                    write!(f, " {v} ")?;
                }
                write!(f, "[")?;
                Ok(())
            }
            crate::Value::Ident(cnt) => write!(f, "{}", cnt),
            crate::Value::Code(cnt) => {
                write!(f, "{cnt}")
            }
        }
    }
}

pub struct DisplayValue(pub crate::Value);
impl Display for DisplayValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            crate::Value::Str(cnt) => write!(f, "{}", cnt),
            crate::Value::Num(cnt) => write!(f, "{}", cnt),
            crate::Value::List(cnt) => {
                write!(f, "[")?;
                for v in cnt {
                    write!(f, " {v} ")?;
                }
                write!(f, "[")?;
                Ok(())
            }
            crate::Value::Ident(cnt) => write!(f, "{}", cnt),
            crate::Value::Code(cnt) => {
                write!(f, "{cnt}")
            }
        }
    }
}

impl Display for crate::parser::Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cont)
    }
}

impl Display for crate::parser::Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'( ")?;
        for expr in &self.exprs {
            write!(f, "{expr} ")?;
        }
        write!(f, ")'")
    }
}

impl Display for crate::parser::FnName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items = self.0.iter().peekable();
        while let Some(i) = items.next() {
            write!(f, "{}", i)?;
            let last = items.peek().is_none();
            if !last {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

impl Display for crate::ExprCont {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::parser::ExprCont::Atom(a) => {
                write!(f, "{a}")
            }
            crate::parser::ExprCont::Code(c) => {
                write!(f, "{c}")
            }
            crate::parser::ExprCont::Macro(m) => {
                write!(f, "{m}")
            }
        }
    }
}

impl Display for crate::parser::MacroUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! ({} {})", self.name, self.content)
    }
}

impl Display for crate::parser::Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ", self.fn_name)?;
        for arg in &self.arguments {
            write!(f, "{arg} ")?;
        }
        write!(f, ")")
    }
}

impl Display for crate::parser::Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::parser::Argument::Atom(a) => write!(f, "{a}"),
            crate::parser::Argument::Macro(m) => write!(f, "{m}"),
            crate::parser::Argument::Value(v) => write!(f, "{v}"),
            crate::parser::Argument::Var(v) => write!(f, "$.{v}"),
        }
    }
}

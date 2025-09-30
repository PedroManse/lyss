pub mod object;

use std::collections::HashMap;

use crate::parser::{Argument, Atom, Expr, FnName};
use crate::{LyssRuntimeError, Value};
use object::*;

pub type HostContext = Context<'static>;
type ParentContext<'p> = &'p Context<'p>;

#[derive(Debug, Clone)]
pub struct HostFunc(
    pub fn(&mut Context, arguments: &[Argument]) -> Result<Value, LyssRuntimeError>,
);

impl HostFunc {
    fn call(self, ctx: &mut Context, arguments: &[Argument]) -> Result<Value, LyssRuntimeError> {
        self.0(ctx, arguments)
    }
}

#[derive(Debug, Default)]
pub struct Context<'p> {
    pub paret: Option<ParentContext<'p>>,
    pub object_store: Object<Value>,
    pub functions: Object<HostFunc>,
    pub variables: HashMap<String, Value>,
}

impl HostContext {
    pub fn new() -> HostContext {
        Context::default()
    }
}

impl<'p> Context<'p> {
    pub fn run(&mut self, code: &[Expr]) -> Result<(), LyssRuntimeError> {
        for expr in code {
            self.execute_expr(expr)?;
        }
        Ok(())
    }
    pub fn register_object(&mut self, name: String, entry: Object<HostFunc>) {
        self.functions.0.insert(name, ObjectEntry::Branch(entry));
    }
    pub fn register_entry(&mut self, name: String, entry: ObjectEntry<HostFunc>) {
        self.functions.0.insert(name, entry);
    }
    pub fn set_var(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
    pub fn get_var(&mut self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    fn execute_expr(&mut self, expr: &Expr) -> Result<Value, LyssRuntimeError> {
        match &expr.cont {
            crate::parser::ExprCont::Atom(atom) => self.execute_atom(atom),
            crate::parser::ExprCont::Code(code) => Ok(Value::Code(code.clone())),
            crate::parser::ExprCont::Macro(m) => todo!("Execute macros {m}"),
        }
    }
    pub fn execute_atom(&mut self, atom: &Atom) -> Result<Value, LyssRuntimeError> {
        self.functions
            .find_leaf(&atom.fn_name.0)?
            .call(self, &atom.arguments)
    }
    pub fn eval_argument(&mut self, argument: &Argument) -> Result<Value, LyssRuntimeError> {
        Ok(match argument {
            Argument::Var(name) => self
                .get_var(name)
                .ok_or(LyssRuntimeError::VarNotFound {
                    name: name.to_string(),
                })?
                .clone(),
            Argument::Atom(atom) => self.execute_atom(atom)?,
            Argument::Value(v) => v.clone(),
            Argument::Macro(m) => todo!("macro argument {m}"),
        })
    }
}

pub mod api;
pub mod object;

use std::collections::HashMap;
use std::rc::Rc;

use crate::parser::{Argument, Atom, Expr};
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
    pub scopes: Vec<Rc<Object<HostFunc>>>,
    pub variables: HashMap<String, Value>,
}

impl HostContext {
    #[must_use]
    pub fn new() -> HostContext {
        Context::default()
    }
}

impl Context<'_> {
    pub fn run(&mut self, code: &[Expr]) -> Result<Option<Value>, LyssRuntimeError> {
        let mut result = Ok(None);
        for expr in code {
            result = Ok(Some(self.execute_expr(expr)?));
        }
        result
    }
    pub fn register(&mut self, name: String, entry: ObjectEntry<HostFunc>) {
        self.functions.0.insert(name, entry);
    }
    pub fn register_object(&mut self, name: String, entry: Object<HostFunc>) {
        self.functions
            .0
            .insert(name, ObjectEntry::Branch(Rc::new(entry)));
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
            //crate::parser::ExprCont::Code(code) => todo!(),
            crate::parser::ExprCont::Macro(m) => todo!("Execute macros {m}"),
        }
    }
    pub fn execute_atom(&mut self, atom: &Atom) -> Result<Value, LyssRuntimeError> {
        for scope in &self.scopes {
            if let Ok(host_fn) = scope.find_leaf(&atom.fn_name.0) {
                return host_fn.call(self, &atom.arguments);
            }
        }
        self.functions
            .find_leaf(&atom.fn_name.0)?
            .call(self, &atom.arguments)
    }
}

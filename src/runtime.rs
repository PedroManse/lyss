pub mod api;
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
    pub aliases: HashMap<String, Vec<String>>,
    pub scopes: Vec<FnName>,
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
            //crate::parser::ExprCont::Code(code) => todo!(),
            crate::parser::ExprCont::Macro(m) => todo!("Execute macros {m}"),
        }
    }
    pub fn execute_atom(&mut self, atom: &Atom) -> Result<Value, LyssRuntimeError> {
        let hook = if atom.fn_name.0[0].starts_with('$') && atom.fn_name.0[0] != "$" {
            let alias_root = &atom.fn_name.0[0];
            let rest = self.aliases[alias_root].to_vec();
            let branch_alias = self.functions.find_branch(&rest)?;
            branch_alias.find_leaf(&atom.fn_name.0[1..])
        } else {
            self.functions.find_leaf(&atom.fn_name.0)
        };

        match hook {
            Ok(hook) => hook.call(self, &atom.arguments),
            Err(err) => {
                for scope in &self.scopes {
                    let branch = self.functions.find_branch(&scope.0)?;
                    let leaf_option = branch.find_leaf(&atom.fn_name.0);
                    if let Ok(leaf_option) = leaf_option {
                        return leaf_option.call(self, &atom.arguments);
                    }
                }
                Err(err)
            }
        }
    }
}


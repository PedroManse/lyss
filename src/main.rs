use std::collections::HashMap;
use std::path::PathBuf;

use lyss::runtime::object::ObjectEntry;
use lyss::runtime::{Context, HostFunc};
use lyss::{LyssRuntimeError, Value};

fn main() {
    let file_name = PathBuf::from("hello.ls");
    let file = std::fs::read_to_string(&file_name).unwrap();
    let o = lyss::tokenizer::tokenize(&file, &file_name);
    let mut i = o.unwrap().into_iter();
    let exprs = lyss::parser::parse(&mut i).unwrap();

    let mut ctx = lyss::runtime::HostContext::new();
    let mut builtins: HashMap<String, ObjectEntry<HostFunc>> = HashMap::new();

    builtins.insert(
        "local".to_string(),
        ObjectEntry::Leaf(HostFunc(|ctx, args| {
            println!("LOCAL BUILTIN EXECUTED");
            for arg in args {
                println!("{arg}");
            }
            Ok(Value::Num(0.0))
        })),
    );

    ctx.register_object(
        "Builtin".to_string(),
        lyss::runtime::object::Object(builtins),
    );
    ctx.run(&exprs).unwrap();
}
